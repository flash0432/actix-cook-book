# HelloActix-Web

일단은 단순히 문자열 hello world를 출력하는 서버를 만들어 보겠습니다.

### ex1

**cargo.toml**

``` toml
[package]
name = "helloworld"
version = "0.1.0"
edition = "2018"

[dependencies]
actix-web = "1.0"
```

**src/main.rs**

``` rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(||HttpResponse::Ok().body("Hello world!")))
    }).bind("127.0.0.1:8080").unwrap().run().unwrap();
}
```

어디선가 많이 본듯한 형태입니다. 
node.js의 express.js 와 형태가 흡사한것을 알수있습니다.

``` javascript
const express = require('express')
const app = express()
const port = 8080

app.get('/', (req, res) => res.send('Hello World!'))
app.listen(port, () => console.log( `Example app listening on port ${port}!` ))
```

또 다른 예제를 보시죠

### ex2
``` toml
[package]
name = "helloworld"
version = "0.1.0"
edition = "2018"

[dependencies]
actix-web = "1.0.5"
serde = "1.0.99"
```

```Rust
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder}; 
use serde::{Deserialize, Serialize}; 
use std::sync:: Mutex; 

struct AppState {

    counter: Mutex<i64>,

}

#[derive(Deserialize)]
struct Name {

    #[serde(default = "Name::name_default_value")]
    name: String,

}
impl Name {

    fn name_default_value() -> String {
        String::from("World!")
    }

}

#[derive(Serialize)]
struct Greeting {

    id: i64,
    content: String,

}

#[get("/greeting")]
fn index(data: web:: Data<AppState>, query: web:: Query<Name>) -> impl Responder {

    let query = &query.name;
    let mut cnt = data.counter.lock().unwrap();
    *cnt += 1;
    
    let greeting = Greeting {
        id: *cnt,
        content: format!("hello, {}", query),
    };
    
    HttpResponse::Ok().json(greeting)

}

fn main() {

    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });
    
    HttpServer::new(move || App::new().register_data(app_state.clone()).service(index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .unwrap();

}
``` 
이형태도 어디선가 본듯한 예제입니다.

``` JAVA
package hello;

import java.util.concurrent.atomic.AtomicLong;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class GreetingController {

    private static final String template = "Hello, %s!";
    private final AtomicLong counter = new AtomicLong();
    
    @RequestMapping("/greeting")
    public Greeting greeting(@RequestParam(value="name", defaultValue="World") String name) {
        return new Greeting(counter.incrementAndGet(),
                            String.format(template, name));
    }
}
```
Spring의 RestApi Greeting 예제와 흡사한것을 알수있습니다.
Rust라는 언어는 JAVA나 Spring 프레임워크보다 나중에 만들어 졌고, 지금까지 이러한 개발에 익숙한 사람들이 있고, 이러한것을 참조해서 개발 하였다는것을 알수 있습니다.
그렇기때문에 Rust언어만 익숙해지신다면, Actix-Web프로젝트의 개발도 하실수 있을 것이라고 생각됩니다.
