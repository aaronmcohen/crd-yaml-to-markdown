# crd-yaml-to-markdown

This is a command line tool to generate Markdown documentation based on an operator's CRD. THis is especially useful for operators that are not written in GO and cannot leverage some of the GO based kube documentation. 

## Getting started
- Download the [latest release](https://github.com/aaronmcohen/crd-yaml-to-markdown/releases) and grant it execution permissions (eg. `chmod +x`).
- execute `crd-yaml-to-markdown-<Your OS> -i <Path to CRD>` which will output the resulting markdown to `stdout`. You may want to pipe the output to a file or `pbcopy` to capture for your `README.md` file.

## Sample

```yaml
---
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
```

Results in: 
### CronTab (stable.example.com/v1)

| Name | Descrption |
| ---- | ---------- |
| cronSpec |  |
| image | Docker image |
| replicas |  |

