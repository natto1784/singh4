use std::process::{Command, ExitStatus};

//Helper lib for running ffmpeg using std::process::Command

fn sanitize_arg<V: Into<String>>(value: V) -> String {
    let v: String = value.into();
    if v.starts_with('-') {
        v
    } else {
        format!("-{}", v)
    }
}

#[derive(Clone, Debug)]
pub struct FfmpegTranscode {
    args: Vec<String>,
    out: String,
    acodec: String,
    vcodec: String,
}

impl Default for FfmpegTranscode {
    fn default() -> Self {
        FfmpegTranscode {
            args: vec![],
            out: String::from("/tmp/ffmpeg"),
            acodec: String::from("copy"),
            vcodec: String::from("copy"),
        }
    }
}

impl FfmpegTranscode {
    pub fn add_flag<V: Into<String>>(&mut self, value: V) -> &mut Self {
        self.args.push(sanitize_arg(value));
        self
    }

    pub fn add_flags<T: Into<String>, V: IntoIterator<Item = T>>(&mut self, value: V) -> &mut Self {
        for v in value {
            self.add_flag(v);
        }
        self
    }

    pub fn add_arg<K: Into<String>, V: ToString>(&mut self, key: K, value: V) -> &mut Self {
        self.args.push(sanitize_arg(key));
        self.args.push(value.to_string());
        self
    }

    pub fn add_input<V: ToString>(&mut self, input: V) -> &mut Self {
        self.add_arg('i', input)
    }

    pub fn set_output<V: ToString>(&mut self, output: V) -> &mut Self {
        self.out = output.to_string();
        self
    }

    pub fn set_acodec<V: ToString>(&mut self, acodec: V) -> &mut Self {
        self.acodec = acodec.to_string();
        self
    }

    pub fn set_vcodec<V: ToString>(&mut self, vcodec: V) -> &mut Self {
        self.vcodec = vcodec.to_string();
        self
    }

    pub fn add_map(&mut self, inp: i32, s: char, out: i32) -> &mut Self {
        self.add_arg("map", format!("{}:{}:{}", inp, s, out))
    }

    pub fn run(&self) -> ExitStatus {
        println!("{}", self.args.join(" "));
        Command::new("ffmpeg")
            .args(&self.args)
            .args(vec!["-c:a", &self.acodec])
            .args(vec!["-c:v", &self.vcodec])
            .arg(&self.out)
            .status()
            .expect("FFmpeg failed to run")
    }
}
