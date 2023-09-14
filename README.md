# rust-demo

Rust demo project with web server and mysql.

This is a background of Resnet algorithm training based on kubernetes. It uses Rust AXUM as the web framework, SQLX as
the database framework, and kubers as the K8s api package. It provides the functions of viewing pods based on http api,
creating pod of Resnet, and viewing logs.

*Please note that you should create your own train data in /bitnami/model-data or specify the mounting directory*
FOr example:
```json
 {
  "apiVersion": "v1",
  "kind": "Pod",
  "metadata": {
    "name": "resnet"
  },
  "spec": {
    "containers": [
      {
        "name": "resnet",
        "image": "bitnami/tensorflow-serving:latest",
        "volumeMounts": [
          {
            "mountPath": "/bitnami/model-data",
            "name": "model-data"
          }
        ]
      }
    ],
    "volumes": [
      {
        "name": "model-data",
        "hostPath": {
          "path": "/root/resnet/model-data"
        }
      }
    ]
  }
}


```


----

### Config

Set your ow config in [.env](.env)

```shell
# Mysql DSN (not used)
DATABASE_URL=mysql://username:password@host:port/dbname

#  Mysql username
DB_USERNAME=username
#  Mysql password
DB_PASSWORD=password
#  Mysql host
DB_HOST=host
#  Mysql port
DB_PORT=port
#  Mysql dbname
DB_NAME=dbname
#  Kubernetes config file ,default is ~/.kube/config if not set
KUBE_CONFIG=kubernetes config file
# Server port ,default is 8080 if not set
SERVER_PORT=8080

```

---

### Api

```curl
# Get pod list
/kube/:namespace/pods

# Create pod,for now it is Resnet pod
/kube/:namespace/pod/create

# Get logs specific pod
/kube/:namespace/pod/logs
```


