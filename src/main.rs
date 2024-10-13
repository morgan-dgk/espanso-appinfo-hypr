//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use std::env;
use std::io::{stdout, Error, Write};
use std::os::unix::net::UnixStream;

#[derive(Serialize, Deserialize, Debug)]

struct AppInfo {
    title: String,
    exec: Option<String>,
    class: String,
}

fn get_socket_addr() -> Result<String, &'static str> {
    let xdg_runtime_dir = match env::var("XDG_RUNTIME_DIR") {
        Ok(val) => val,
        Err(_err) => return Err("$XDG_RUNTIME_DIR not set"),
    };

    let hypr_instance_sig = match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(val) => val,
        Err(_err) => return Err("$HYPRLAND_INSTANCE_SIGNATURE not set"),
    };

    let socket_addr = format!("{xdg_runtime_dir}/hypr/{hypr_instance_sig}/.socket.sock");

    Ok(socket_addr)
}

fn main() -> std::io::Result<()> {
    let socket_addr = match get_socket_addr() {
        Err(err) => return Err(Error::other(err)),
        Ok(addr) => addr,
    };

    let mut stream = UnixStream::connect(socket_addr)?;
    stream.write_all(b"j/activewindow")?;

    let app_info: AppInfo = serde_json::from_reader(stream)?;

    let out = serde_json::to_string(&app_info)?;

    stdout().write_all(out.as_bytes())?;

    Ok(())
}
