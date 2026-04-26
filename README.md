# winventory-rs

`winventory-rs` is a small Rust-based Windows asset inventory tool for defensive security assessment.

It collects basic local host metadata and prints it as JSON:

- hostname
- Windows product name
- Windows display version / release ID
- Windows build number
- installed browser versions for Chrome, Edge, and Firefox where available

## Purpose

The tool is designed for small-scale defensive security assessment and lab work. It provides a lightweight way to collect basic OS and browser version information from a Windows host without installing an agent or collecting sensitive user data.

This kind of inventory is useful for:

- identifying outdated operating system builds
- identifying outdated browser versions
- basic asset visibility
- small-organisation security reviews
- lab exercises around local enumeration and defensive telemetry

## Privacy and Scope

This tool does **not** collect:

- credentials
- documents
- browser history
- cookies
- saved passwords
- network traffic
- personal files

It only queries local OS and browser version metadata.

## How It Works

The tool uses standard Windows mechanisms:

- `reg query` for registry-based version information
- PowerShell `Get-Item` for executable product versions when registry values are unavailable
- Rust `std::process::Command` to invoke local system commands
- JSON output for easy downstream processing

## Example Output

```json
{
  "hostname": "DESKTOP-EXAMPLE",
  "os": {
    "product_name": "Windows 11 Pro",
    "display_version": "23H2",
    "build": "22631"
  },
  "browsers": {
    "chrome": "124.0.6367.91",
    "edge": "124.0.2478.67",
    "firefox": null
  }
}
## 5. Add a license

MIT is fine for this.

```bash
cat > LICENSE <<'EOF'
MIT License

Copyright (c) 2026 SegmondFault

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell   
copies of the Software, and to permit persons to whom the Software is      
furnished to do so, subject to the following conditions:                   

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.                            

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR    
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,      
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE   
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER        
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, 
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE 
SOFTWARE.
