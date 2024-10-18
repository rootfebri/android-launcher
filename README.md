# Run or Select and RUN android emulator
### Windows only

ENSURE `ANDROID_HOME` is set and have `$ANDROID_HOME/emulator/emulator.exe` installed

### Linux (_Coming soon_)

```shell
cargo build -r
copy target/release/android-launcher.exe .
.\android-launcher.exe
```
![Screenshot](./image/Screenshot.png)