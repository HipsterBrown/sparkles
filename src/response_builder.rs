use response::Response;
use response::BTreeMap;
use status::Status;

pub struct ResponseBuilder {
    pub data: BTreeMap,
    template: Option<String>,
    status: Option<Status>,
}

impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {
            data: BTreeMap::new(),
            template: None,
            status: None,
        }
    }

    pub fn with_template(&mut self, template: String) {
        self.template = Some(template);
    }

    pub fn with_status(&mut self, status: Status) {
        self.status = Some(status);
    }

    pub fn to_response(self) -> Response {
        Response {
            data: self.data,
            template: self.template.unwrap(),
            status: self.status.unwrap(),
        }
    }
}
