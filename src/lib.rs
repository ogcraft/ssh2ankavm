use std::error::Error;
//extern crate regex;
use regex::Regex;
use std::process::Command;

#[derive(Debug)]
pub struct Config {
    pub vm_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        println!("args: {:?}", args);
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let vm_name = args[1].clone();

        Ok(Config { vm_name: vm_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Config: {:?}", config);
    let ifconfig_output = run_ifconfig(&config.vm_name);
    println!("ifconfig_output:[{:?}]", ifconfig_output);
    let re = Regex::new(r"^\s*inet\s+(?P<ip>(?:[0-9]{1,3}\.){3}[0-9]{1,3}) netmask.*broadcast.*$")
        .unwrap();
    let caps = ifconfig_output
        .lines()
        .filter_map(|line| re.captures(line))
        .nth(0)
        .unwrap();
    let ip_addr = caps.name("ip").unwrap().as_str();
    println!("caps: {:?}", ip_addr);
    let _output = Command::new("/usr/bin/ssh")
        .args(&[format!("admin@{}", ip_addr)])
        .output()
        .expect("failed to execute ssh process");
    Ok(())
}

pub fn run_ifconfig(vm_name: &str) -> String {
    println!("run_ifconfig: {:?}", vm_name);
    let ifconfig_pfg = "/usr/local/bin/anka";
    let ifconfig_args = ["run", "-n", vm_name, "ifconfig"];
    let output = Command::new(ifconfig_pfg)
        .args(&ifconfig_args)
        .output()
        .expect("failed to execute ifconfig process");

    String::from_utf8(output.stdout).unwrap()
}

///////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::process::Command;

    #[test]
    fn check_config_pass() {
        let args = ["prgname".to_string(), "vm_name".to_string()];
        let config = Config::new(&args);
        println!("config: {:?}", config);
        //assert_eq!(config.vm_name, "vm_name");
        assert_eq!(0, 0);
    }

    #[test]
    fn check_config_fail() {
        let args = ["prgname".to_string()];
        let config = Config::new(&args);
        println!("config: {:?}", config);
        //assert_eq!(config.vm_name, "vm_name");
        assert_eq!(0, 0);
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn check_run_ifconfig() {
        let ifconfig_pfg = "/usr/local/bin/anka";
        let ifconfig_args = ["run", "-n", "catalina_10.15_test", "ifconfig"];
        let output = Command::new(ifconfig_pfg)
            .args(&ifconfig_args)
            .output()
            .expect("failed to execute ifconfig process");

        let output_str = String::from_utf8(output.stdout).expect("Cannot convert output");
        let output_lines: Vec<&str> = output_str.lines().collect();

        println!("output: [{:?}]", output_lines);

        output_lines.iter().for_each(|l| println!("{:?}", l));

        assert_eq!(0, 0);
    }
    #[test]
    fn check_parse_ifconfig() {
        let ifconfig_otput = "lo0: flags=8049<UP,LOOPBACK,RUNNING,MULTICAST> mtu 16384
        options=1203<RXCSUM,TXCSUM,TXSTATUS,SW_TIMESTAMP>
        inet 127.0.0.1 netmask 0xff000000
        inet6 ::1 prefixlen 128
        inet6 fe80::1%lo0 prefixlen 64 scopeid 0x1
        nd6 options=201<PERFORMNUD,DAD>
    gif0: flags=8010<POINTOPOINT,MULTICAST> mtu 1280
    stf0: flags=0<> mtu 1280
    XHC7: flags=0<> mtu 0
    en0: flags=8863<UP,BROADCAST,SMART,RUNNING,SIMPLEX,MULTICAST> mtu 1500
        options=423<RXCSUM,TXCSUM,TSO4,CHANNEL_IO>
        ether 56:45:45:67:8c:df
        inet6 fe80::808:e2d4:3d15:5b23%en0 prefixlen 64 secured scopeid 0x5
        inet 192.168.64.24 netmask 0xffffff00 broadcast 192.168.64.255
        nd6 options=201<PERFORMNUD,DAD>
        media: 1000baseT <full-duplex>
        status: active
    utun0: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1380
        inet6 fe80::418c:b5dc:c458:32c2%utun0 prefixlen 64 scopeid 0x6
        nd6 options=201<PERFORMNUD,DAD>
    utun1: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 2000
        inet6 fe80::adf2:9b24:f441:6b41%utun1 prefixlen 64 scopeid 0x7
        nd6 options=201<PERFORMNUD,DAD>";

        let ifconfig_output_str = ifconfig_otput.to_string();

        let output_lines: Vec<&str> = ifconfig_output_str.lines().collect();

        println!("output: [{:?}]", output_lines);

        output_lines.iter().for_each(|l| println!("{:?}", l));

        assert_eq!(0, 0);
    }

    #[test]
    fn test_inet_regex() {
        let re =
            Regex::new(r"^\s*inet\s+(?P<ip>(?:[0-9]{1,3}\.){3}[0-9]{1,3}) netmask.*broadcast.*$")
                .unwrap();
        let test_str = "inet 192.168.64.24 netmask 0xffffff00 broadcast 192.168.64.255";
        assert!(re.is_match(test_str));
        let caps = re.captures(test_str).unwrap();
        let ip_addr = caps.name("ip").unwrap().as_str();
        println!("ip_addr: {:?}", ip_addr);
        assert_eq!(ip_addr, "192.168.64.24");
    }

    #[test]
    fn test_ssh_string() {
        let test_str = format!("admin@{}", "127.0.0.1");
        assert_eq!("admin@127.0.0.1", test_str);
    }
}
/*
anka run  -n catalina_10.15_test ifconfig

lo0: flags=8049<UP,LOOPBACK,RUNNING,MULTICAST> mtu 16384
    options=1203<RXCSUM,TXCSUM,TXSTATUS,SW_TIMESTAMP>
    inet 127.0.0.1 netmask 0xff000000
    inet6 ::1 prefixlen 128
    inet6 fe80::1%lo0 prefixlen 64 scopeid 0x1
    nd6 options=201<PERFORMNUD,DAD>
gif0: flags=8010<POINTOPOINT,MULTICAST> mtu 1280
stf0: flags=0<> mtu 1280
XHC7: flags=0<> mtu 0
en0: flags=8863<UP,BROADCAST,SMART,RUNNING,SIMPLEX,MULTICAST> mtu 1500
    options=423<RXCSUM,TXCSUM,TSO4,CHANNEL_IO>
    ether 56:45:45:67:8c:df
    inet6 fe80::808:e2d4:3d15:5b23%en0 prefixlen 64 secured scopeid 0x5
    inet 192.168.64.24 netmask 0xffffff00 broadcast 192.168.64.255
    nd6 options=201<PERFORMNUD,DAD>
    media: 1000baseT <full-duplex>
    status: active
utun0: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1380
    inet6 fe80::418c:b5dc:c458:32c2%utun0 prefixlen 64 scopeid 0x6
    nd6 options=201<PERFORMNUD,DAD>
utun1: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 2000
    inet6 fe80::adf2:9b24:f441:6b41%utun1 prefixlen 64 scopeid 0x7
    nd6 options=201<PERFORMNUD,DAD>
*/
