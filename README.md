practice 1
================================
`https://guptanikhil.medium.com/rust-wasm-breakout-game-i-1b474e41a132`


practice 2
================================
`https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=1&pp=iAQB`


practice 3
================================
`https://www.youtube.com/watch?v=izhFutJiZgo&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=2&pp=iAQB`


practice 4
================================
`https://www.youtube.com/watch?v=pZRzZiJjJ1I&list=PLp0sjyxOq4ATFgiJ4HL8ok9Yp2h7Hz1Fb&index=3`


bevy
================================
`cargo add wasm-bindgen`

`cargo add bevy`

`cargo add bevy_third_person_camera`

`cargo add bevy-inspector-egui`


WASM Build
================================
- Cargo.toml 디렉토리 경로 : wasm 디렉토리 있어야됨

`cargo build --target wasm32-unknown-unknown --release`

`wasm-bindgen target/wasm32-unknown-unknown/release/{wasm 이름}.wasm --out-dir wasm --target web`




asp.net core 6 cshtml 에서 gltf 확장자 파일 로드 안될때
================================
program.cs

```
var provider = new FileExtensionContentTypeProvider();
provider.Mappings[".gltf"] = "model/gltf+json";
app.UseStaticFiles(new StaticFileOptions
{
    ContentTypeProvider = provider
});
```