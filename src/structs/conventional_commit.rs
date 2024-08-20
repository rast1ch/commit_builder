#[derive(Debug)]
pub struct ConventionalCommit {
    type_: String,
    scope: String,
    description: String,
    body: String,
    wip: bool,
}

impl ConventionalCommit {
    fn new(type_: String, scope: String, description: String, body: String, wip: bool) -> Self {
        Self {
            type_,
            scope,
            description,
            body,
            wip,
        }
    }

    fn print(&self) {
        println!("type: {}", self.type_);
        println!("scope: {}", self.scope);
        println!("description: {}", self.description);
        println!("body: {}", self.body);
        println!("wip: {}", self.wip);
    }

    fn is_wip(&self) -> bool {
        self.wip
    }

    fn build(&self) -> String {
        let mut commit = String::new();
        commit.push_str(&self.type_);
        if !self.scope.is_empty() {
            commit.push_str("(");
            commit.push_str(&self.scope);
            commit.push_str("): ");
        }
        commit.push_str(&self.description);
        if self.wip {
            commit.push_str(" wip.");
        }
        commit.push_str("\n\n");
        commit.push_str(&self.body);
        commit
    }
}
