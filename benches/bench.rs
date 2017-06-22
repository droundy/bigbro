#![feature(test)]

extern crate test;
extern crate bigbro;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn status_true(b: &mut Bencher) {
        b.iter(|| {
            bigbro::Command::new("true").status()
        });
    }

    #[bench]
    fn spawn_hook_true(b: &mut Bencher) {
        b.iter(|| {
            let (tx,rx) = std::sync::mpsc::sync_channel(1);
            let cmd = bigbro::Command::new("true");
            let _killer = cmd.spawn_and_hook(move |s| { tx.send(s).ok(); })
                .expect("failed to execute true");
            rx.recv()
        });
    }

    #[bench]
    fn blind_spawn_hook_true(b: &mut Bencher) {
        b.iter(|| {
            let (tx,rx) = std::sync::mpsc::sync_channel(1);
            let mut cmd = bigbro::Command::new("true");
            cmd.blind(true);
            let _killer = cmd.spawn_and_hook(move |s| { tx.send(s).ok(); })
                .expect("failed to execute true");
            rx.recv()
        });
    }

    #[bench]
    fn status_echo(b: &mut Bencher) {
        b.iter(|| {
            bigbro::Command::new("echo").arg("-n").save_stdouterr().status()
        });
    }

    #[bench]
    fn spawn_hook_echo(b: &mut Bencher) {
        b.iter(|| {
            let (tx,rx) = std::sync::mpsc::sync_channel(1);
            let mut cmd = bigbro::Command::new("echo");
            cmd.arg("-n").arg("hello").save_stdouterr();
            let _killer = cmd.spawn_and_hook(move |s| { tx.send(s).ok(); })
                .expect("failed to execute echo");
            rx.recv()
        });
    }

    #[bench]
    fn blind_spawn_hook_echo(b: &mut Bencher) {
        b.iter(|| {
            let (tx,rx) = std::sync::mpsc::sync_channel(1);
            let mut cmd = bigbro::Command::new("echo");
            cmd.arg("-n").arg("hello").save_stdouterr().blind(true);
            let _killer = cmd.spawn_and_hook(move |s| { tx.send(s).ok(); })
                .expect("failed to execute echo");
            rx.recv()
        });
    }
}
