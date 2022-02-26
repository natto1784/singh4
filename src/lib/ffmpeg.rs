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
    acodec: Option<String>,
    vcodec: Option<String>,
    abitrate: Option<String>,
    vbitrate: Option<String>,
    afilter: Vec<String>,
    vfilter: Vec<String>,
}

impl Default for FfmpegTranscode {
    fn default() -> Self {
        FfmpegTranscode {
            args: vec![],
            out: String::from("/tmp/ffmpeg"),
            acodec: None,
            vcodec: None,
            abitrate: None,
            vbitrate: None,
            afilter: vec![],
            vfilter: vec![],
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
        self.acodec = Some(acodec.to_string()).filter(String::is_empty);
        self
    }

    pub fn set_vcodec<V: ToString>(&mut self, vcodec: V) -> &mut Self {
        self.vcodec = Some(vcodec.to_string()).filter(String::is_empty);
        self
    }

    pub fn set_abitrate<V: ToString>(&mut self, bit: V) -> &mut Self {
        self.abitrate = Some(bit.to_string()).filter(String::is_empty);
        self
    }

    pub fn set_vbitrate<V: ToString>(&mut self, bit: V) -> &mut Self {
        self.vbitrate = Some(bit.to_string()).filter(String::is_empty);
        self
    }

    pub fn add_afilter<V: Into<String>>(&mut self, value: V) -> &mut Self {
        self.afilter.push(value.into());
        self
    }

    pub fn set_afilter<T: Into<String>, V: IntoIterator<Item = T>>(
        &mut self,
        value: V,
    ) -> &mut Self {
        self.afilter = value.into_iter().map(|x| x.into()).collect();
        self
    }

    pub fn add_vfilter<V: Into<String>>(&mut self, value: V) -> &mut Self {
        self.vfilter.push(value.into());
        self
    }

    pub fn set_vfilter<T: Into<String>, V: IntoIterator<Item = T>>(
        &mut self,
        value: V,
    ) -> &mut Self {
        self.vfilter = value.into_iter().map(|x| x.into()).collect();
        self
    }

    pub fn add_map(&mut self, inp: i32, s: char, out: i32) -> &mut Self {
        self.add_arg("map", format!("{}:{}:{}", inp, s, out))
    }

    pub fn run(&self) -> ExitStatus {
        println!("{}", self.args.join(" "));

        Command::new("ffmpeg")
            .args(&self.args)
            .args(match &self.acodec {
                Some(ac) => vec!["-c:a", ac],
                None => vec![],
            })
            .args(match &self.vcodec {
                Some(vc) => vec!["-c:v", vc],
                None => vec![],
            })
            .args(match &self.abitrate {
                Some(ab) => vec!["-b:a", ab],
                None => vec![],
            })
            .args(match &self.vbitrate {
                Some(vb) => vec!["-b:v", vb],
                None => vec![],
            })
            .args(if !self.vfilter.is_empty() {
                vec![String::from("-filter:v"), self.vfilter.join(";")]
            } else {
                vec![]
            })
            .args(if !self.afilter.is_empty() {
                vec![String::from("-filter:a"), self.afilter.join(";")]
            } else {
                vec![]
            })
            .arg(&self.out)
            .status()
            .expect("Can't execute ffmpeg")
    }
}
