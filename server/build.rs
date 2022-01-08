use std::process::Command;

pub fn exec<const N: usize>(command: [&str; N]) {
    let output = Command::new(command[0])
        .args(&command[1..])
        .output()
        .unwrap();
    if !output.status.success() {
        println!(
            "cargo:warning=command {} exited with code {}",
            command.join(" "),
            output.status.code().unwrap_or(-1)
        );
    }
}

#[rustfmt::skip]
pub fn main() {
    exec(["rm", "-rf", "public"]);
    exec(["cp", "-a", "src/public", "public"]);
    exec(["npx", "tailwindcss", "-i", "src/public/index.css", "-o", "public/index.css", "-m"]);
}
