#[macro_use]
extern crate getset;

#[derive(Default, Debug, Getters, PartialEq, Eq)]
pub struct HtttpResponse {
    #[getset(get = "pub")]
    status: String,
    #[getset(get = "pub")]
    message: String,
    #[getset(get = "pub")]
    version: String,
    #[getset(get = "pub")]
    field: Vec<String>,
    #[getset(get = "pub")]
    body: String,
}

impl HtttpResponse {
    pub fn new() -> HtttpResponse {
        Default::default()
    }

    pub fn set_status_code(&mut self, status: &str) -> &mut Self{
        self.status = String::from(status);

        self
    }

    pub fn set_message(&mut self, message: &str) -> &mut Self{
        self.message = String::from(message);

        self
    }

    pub fn set_version(&mut self, version: &str) -> &mut Self{
        self.version = String::from(version);

        self
    }

    pub fn set_body(&mut self, body: &str) -> &mut Self{
        self.body = String::from(body);

        self
    }

    pub fn push_field_line(&mut self, field_line: &str) -> &mut Self{
        self.field.push(String::from(field_line));

        self
    }

    pub fn append_field(&mut self, field: &mut Vec<String>) -> &mut Self{
        self.field.append(field);

        self
    }

    pub fn is_empty(&self) -> bool {
        self.status == "" ||
        self.message == "" ||
        self.version == "" ||
        self.field.len() == 0 ||
        self.body == ""
    }

    pub fn is_valid(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        else{
            return true;
        }
    }

}

impl std::fmt::Display for HtttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
        let mut request = format!("{} {} {}", self.version, self.status, self.message);

        for field_line in self.field.clone() {
            request = request + "\r\n" + &field_line;
        }

        request = format!("{}\r\n\r\n{}", request, self.body);

        write!(f, "{}", request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_build_http_responce() {
        let response_base = "HTTP/1.1 200 OK\r\n\
            Content-Length: 299\r\n\r\n\
            function test(e) {\r\n\
            \tconsole.log(e);\r\n\
            }\r\n\
            \r\n\
            // Add event listener on keydown\r\n\
            document.addEventListener('keydown', (event) => {\r\n\
            \tvar name = event.key;\r\n\
            \tvar code = event.code;\r\n\
            \t// Alert the key name and key code on keydown\r\n\
            \tconsole.log(`Key pressed ${name} \r\n Key code value: ${code}`);\r\n\
            \r\n\
            }, false);";

        let mut response = HtttpResponse::new();

        response.set_version("HTTP/1.1")
        .set_status_code("200")
        .set_message("OK")
        .push_field_line("Content-Length: 299")
        .set_body("function test(e) {\r\n\
            \tconsole.log(e);\r\n\
            }\r\n\
            \r\n\
            // Add event listener on keydown\r\n\
            document.addEventListener('keydown', (event) => {\r\n\
            \tvar name = event.key;\r\n\
            \tvar code = event.code;\r\n\
            \t// Alert the key name and key code on keydown\r\n\
            \tconsole.log(`Key pressed ${name} \r\n Key code value: ${code}`);\r\n\
            \r\n\
            }, false);");

        assert_eq!(response_base, format!("{}",response));
    }
}
