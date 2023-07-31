Steps:

yarn create react-app frontend --template typescript

create frontend docker file

create nginx.conf


cargo new --bin server

create server dockerfile

Create helm charts: 

helm create server
helm create frontend


create simple skaffold.yaml by hand

create .dockerignore



#TODO create a migration container.  This will be a single job that is run in skaffold before server starts.  this should be part of CI in production


To run:

minikube start
minikube addons enable ingress
skaffold dev