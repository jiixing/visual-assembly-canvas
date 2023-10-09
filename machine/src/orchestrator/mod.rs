use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use crate::{Machine, Parser, Execute, Message, Action, MAPPED_START};

pub struct Orchestrator {
    pub machines: Vec<Machine>,
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>,
}

impl Orchestrator {
    pub fn new() -> Orchestrator {
        let (tx, rx) = mpsc::channel();

        Orchestrator {
            machines: vec![],
            tx,
            rx,
        }
    }

    pub fn add(&mut self) -> u16 {
        let mut m = Machine::new();
        let id = self.machine_id();
        m.id = Some(id);

        {
            let tx = self.tx.clone();

            m.handlers.message = Some(Box::new(move |m| {
                println!("on_message: {:?}", m);
                tx.clone().send(m).unwrap();
            }));
        };

        self.machines.push(m);
        id
    }

    pub fn machine_id(&self) -> u16 {
        self.machines.len() as u16
    }

    pub fn run_code(&mut self, id: u16, code: &str) {
        let machine = &mut self.machines[id as usize];

        let parser: Parser = code.into();
        machine.mem.load_symbols(parser.symbols);
        machine.mem.load_code(parser.ops);
        machine.run();
    }

    pub fn read_packet(&mut self) {
        let Message { from, to, action } = self.rx.recv().unwrap();

        match action {
            Action::Data { body } => {
                println!("broker#recv: from={} to={} body={:?}", from, to, body);

                if let Some(m) = self.machines.get_mut(to as usize) {
                    let id = m.id.unwrap_or(0);

                    // The message landed on the wrong machine.
                    if id != to { return; }

                    println!("recv at {}: {:?}", id, body);
                    m.mem.write(MAPPED_START, &body);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MAPPED_START, Orchestrator};

    #[test]
    fn test_send_message() {
        let mut o = Orchestrator::new();

        let m1_id = o.add();
        let m2_id = o.add();

        o.run_code(m1_id, r"
            push 0xDEAD
            push 0xBEEF
            send 0x01 0x02

            push 0b1100
            push 0b1010
            send 0x01 0x02
        ");

        o.read_packet();
        let m2 = o.machines.get_mut(m2_id as usize).unwrap();
        assert_eq!(m2.mem.read(MAPPED_START, 2), [0xBEEF, 0xDEAD]);

        o.read_packet();
        let m2 = o.machines.get_mut(m2_id as usize).unwrap();
        assert_eq!(m2.mem.read(MAPPED_START, 2), [10, 12]);
    }
}