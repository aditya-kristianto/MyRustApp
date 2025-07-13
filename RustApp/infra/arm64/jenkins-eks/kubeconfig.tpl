apiVersion: v1
clusters:
- cluster:
    server: ${endpoint}
    certificate-authority-data: ${certificate}
  name: ${cluster_name}
contexts:
- context:
    cluster: ${cluster_name}
    user: ${cluster_name}-user
  name: ${cluster_name}
current-context: ${cluster_name}
kind: Config
preferences: {}
users:
- name: ${cluster_name}-user
  user:
    token: ${token}
