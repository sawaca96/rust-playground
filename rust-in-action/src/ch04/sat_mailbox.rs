#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroudStation;

impl GroudStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

pub fn run() {
    let base = GroudStation;
    let mut sat_a = CubeSat {
        id: 1,
        mailbox: Mailbox { messages: vec![] },
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("Hello, CubeSat A!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}
