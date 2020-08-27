use std::process::{Command, Stdio};

pub mod sms {
    pub fn send() {}
}

pub mod email {
    use crate::parse;
    const to: &str = "a.barthleemy@cil-lamballe.com";

    pub fn send(user: &str, info: &parse::UserInfo, act: &str) {
        let ssmtp = "ssmtp ".to_string()
            + to
            + &format!(
                " <<< \"{}\"",
                format!(
                    "Subject: {}\n{}\n",
                    format!("{} - {}", user, act), format!("{} performed {}\nDetail:\n{:?}", user, act, info)
                )
            );

        println!("{}", ssmtp);

        let output = std::process::Command::new("bash")
            .arg("-c")
            .arg(ssmtp)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        // Debug
//        println!("status: {}", output.status);
//        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
