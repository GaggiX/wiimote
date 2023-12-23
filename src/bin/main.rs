use wiimote::{Action, IRMode, IRSensitivity, ReportingMode, Wiimote};

fn main() {
    let w = Wiimote::open();
    let a = std::env::args().nth(1);
    
    println!("{:?}", w.len());
    //if let Some(a) = a {
    //    match a.as_str() {
    //        "basic_ir" => {
    //            w.write(Action::IRCameraEnable(Some((
    //                IRMode::Basic,
    //                IRSensitivity::Level3,
    //            ))));
    //            w.write(Action::SetReporting(ReportingMode::ButtonsAccelIR10Ext6));
    //        }
    //        "accel" => {
    //            w.write(Action::SetReporting(ReportingMode::ButtonsAccel));
    //        }
    //        _ => panic!("unknown mode"),
    //    }
    //}
    //loop {
    //    eprintln!("{:?}", w.read());
    //}
}
