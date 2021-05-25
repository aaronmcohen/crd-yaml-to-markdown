use crd_yaml_to_markdown;
use std::error::Error;
use pretty_assertions::assert_eq;

const YAML: &str  = r##"
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  # name must match the spec fields below, and be in the form: <plural>.<group>
  name: crontabs.stable.example.com
spec:
  # group name to use for REST API: /apis/<group>/<version>
  group: stable.example.com
  # list of versions supported by this CustomResourceDefinition
  versions:
    - name: v1
      # Each version can be enabled/disabled by Served flag.
      served: true
      # One and only one version must be marked as the storage version.
      storage: true
      schema:
        openAPIV3Schema:
          type: object
          properties:
            spec:
              type: object
              properties:
                cronSpec:
                  type: string
                image:
                  type: string
                  description: Docker image
                replicas:
                  type: integer
  # either Namespaced or Cluster
  scope: Namespaced
  names:
    # plural name to be used in the URL: /apis/<group>/<version>/<plural>
    plural: crontabs
    # singular name to be used as an alias on the CLI and for display
    singular: crontab
    # kind is normally the CamelCased singular type. Your resource manifests use this.
    kind: CronTab
    # shortNames allow shorter string to match your resource on the CLI
    shortNames:
    - ct
"##;

const MARKDOWN: &str = r##"### CronTab (stable.example.com/v1)

| Name | Descrption |
| ---- | ---------- |
| cronSpec |  |
| image | Docker image |
| replicas |  |
"##;

#[test]
fn yaml_to_markdown(){
    let result:Result< String,Box<dyn Error> > = crd_yaml_to_markdown::yaml_to_markdown(YAML);
    assert!(!result.is_err(),"Error was not thrown.");
    assert_eq!(result.unwrap(), MARKDOWN);
}
