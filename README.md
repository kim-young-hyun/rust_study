# Rust Study

## 1. Rust 설치

1. **Visual Studio download**

   Rust를 윈도우에서 실행시키려면 ms c++ build tool이 필요하다.

   설치를 위해 Visual Studio(https://visualstudio.microsoft.com/ko/)을 설치한다.

   Visual Studio Installer에서 [C++을 사용한 데스크톱 개발], [.NET 데스크톱 빌드 도구], [유니버셜 Windows 플랫폼 빌드 도구] 선택한다.

2. **Rust download**

   Rust 공식사이트(https://www.rust-lang.org/)에서 Rust를 설치한다.

3. **visual studio code download**

   개발에 사용할 IDE는 VSC에 확장 기능을 이용한다.

   VSC 확장 기능에서 [rust-analyer]를 설치 (한국어 팩인 [korean Language Pack for Visual Studio Code]도 추천)

4. **Hello World 출력**

​	새로운 폴더, 파일 생성: Hello/hello.rs (폴더 만드는 이유는 실행파일도 생기면 복잡할 꺼 같아서)		

```rust
fn main() {
	println!("Hello World!");
}
```

​	코드 작성하고 터미널 열기		

```
 rustc hello.rs //컴파일

./hello // 실행
```

​	.rs 파일을 컴파일 하면 .pdb 파일과 .exe 파일이 생성된다.

​	.pdb는 Programming Debug Database로 윈도우에서 디버깅용 정보가 들어있다.