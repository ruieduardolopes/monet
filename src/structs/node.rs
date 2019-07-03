#[derive(Debug)]
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
            .collect();
        let egress_interfaces: Vec<String> = egress
            .split(",")
            .map(|element| element.to_string())
            .collect();

        // Create the structure as is with the given data.
        Self {
            ingress_interfaces,
            egress_interfaces,
        }
    }
}
