use std::fs;
use std::path::Path;
use toml::Value;

fn main() {
    let args = Args::new();
    println!("file upload [{}] => [{}]", args.lf.as_str(), args.c.remote_path().as_str());
    let mut session = ssh_rs::ssh::create_session();
    session.set_user_and_password(args.c.user(), args.c.password());
    let addr = format!("{}:{}", args.c.ip(), args.c.port());
    session.connect(addr).unwrap();
    let scp = session.open_scp().unwrap();
    scp.upload(args.lf.as_str(), args.c.remote_path().as_str()).unwrap();
    session.close().unwrap();
    println!("file upload successfully.");
}



#[derive(Debug)]
struct Args {
    c: Conf,
    lf: String,
}

impl Args  {
    pub fn new() -> Self {
        let mut args = std::env::args();
        let exe_path = args.next().unwrap();
        let p = Path::new(&exe_path);
        let c = p.parent().unwrap().parent().unwrap();
        let mut path_buf = c.join("config").join("conf.toml");
        let mut lf = String::new();

        if args.len() <= 0 {
            let conf = Conf::from(path_buf.as_path());
            lf = conf.local_path();
            return Args {
                c: conf,
                lf
            }
        }

        loop {
            let arg = args.next();
            if arg.is_none() {
                break;
            }
            match arg.unwrap().as_str() {
                "c" => {
                    if let Some(p) = args.next() {
                        path_buf = Path::new(p.as_str()).to_path_buf()
                    }
                },
                "lf" => {
                    if let Some(lfs) = args.next() {
                        lf = lfs;
                    }
                }
                _ => {}
            }
        }

        let conf = Conf::from(path_buf.as_path());
        if lf.is_empty() {
            lf = conf.local_path();
        }
         Args {
            c: conf,
            lf
         }
    }
}


#[derive(Debug)]
struct Conf {
    v: Value
}

impl Conf {
    fn from(path: &Path) -> Self {
        let ts = fs::read_to_string(path.to_path_buf()).unwrap();
        let v: Value = toml::from_str(ts.as_str()).unwrap();
        Conf { v }
    }
    fn ip(&self) -> String {
        let ip = self.v.get("ip").unwrap();
        ip.as_str().unwrap().to_string()
    }
    fn port(&self) -> u32 {
        let port = self.v.get("port").unwrap();
        port.as_integer().unwrap() as u32
    }
    fn user(&self) -> String {
        let user = self.v.get("user").unwrap();
        user.as_str().unwrap().to_string()
    }
    fn password(&self) -> String {
        let password = self.v.get("password").unwrap();
        password.as_str().unwrap().to_string()
    }
    fn local_path(&self) -> String {
        let local_path = self.v.get("local_path").unwrap();
        local_path.as_str().unwrap().to_string()
    }
    fn remote_path(&self) -> String {
        let remote_path = self.v.get("remote_path").unwrap();
        remote_path.as_str().unwrap().to_string()
    }
}