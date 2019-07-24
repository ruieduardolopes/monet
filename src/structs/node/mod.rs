#[derive(Clone, Debug)]
pub struct Node {
    ingress_interfaces: Vec<String>,
    egress_interfaces: Vec<String>,
}

impl Node {
    pub fn new(ingress: String, egress: String) -> Self {
        // Remove whitespaces in both ingress and egress arguments.
        ingress.clone().retain(|character| character != ' ');
        egress.clone().retain(|character| character != ' ');

        // Create vectors by splitting the latter strings by commas.
        let ingress_interfaces: Vec<String> = ingress
            .split(",")
            .map(|element| element.to_string())
            .filter(|element| element.len() > 0)
            .collect();
        let egress_interfaces: Vec<String> = egress
            .split(",")
            .map(|element| element.to_string())
            .filter(|element| element.len() > 0)
            .collect();

        // Create the structure as is with the given data.
        Self {
            ingress_interfaces,
            egress_interfaces,
        }
    }

    pub fn status(&self) -> NodeStatus {
        
    }

    pub fn ingress(&self) -> Vec<String> {
        self.ingress_interfaces.clone()
    }

    pub fn egress(&self) -> Vec<String> {
        self.egress_interfaces.clone()
    }

    pub fn all(&self) -> Vec<(String, bool)> {
        let mut interfaces: Vec<(String, bool)> = Vec::new();
        self.ingress_interfaces
            .clone()
            .into_iter()
            .for_each(|element| interfaces.push((element, true)));
        self.egress_interfaces
            .clone()
            .into_iter()
            .for_each(|element| interfaces.push((element, false)));
        interfaces
    }
}