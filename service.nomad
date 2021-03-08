job "campip" {
  datacenters = ["darkstar"]

  type = "service"

  group "campip-app" {
    count = 1
    network {
      port "app"  {
        to = 7800
      }
    }

    service {
      name = "campip-app"
      tags = ["campid", "campid-campip", "campip", "app"]
      port = "app"
    }

    task "web-app" {
      driver = "docker"

      env {
        POSTGRES_USER = "root"
        POSTGRES_DATABSE = "campip_db"
        POSTGRES_PASSWORD = "campip"
      }

      resources {
        cpu = 600
        memory = 512
      }

      config {
        image = "ewestudio/campip:latest"
        ports = ["app"]

      }
    }

  }
  group "campip-db" {
    count = 1

    network {
      port "db" {
        to = 5432
      }
    }

    service {
      name = "campip-db"
      tags = ["campid", "campip", "db"]
      port = "db"
    }

    task "web-db" {
      driver = "docker"

      env {
        POSTGRES_DATABSE = "campip_db"
        POSTGRES_USER = "root"
        POSTGRES_PASSWORD = "campip"
      }

      resources {
        cpu = 600
        memory = 512
      }

      config {
        image = "postgres:13.2"
        ports = ["db"]
      }
    }
  }

}
