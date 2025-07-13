output "cluster_name" {
  description = "The Cluster Name of the EKS Cluster"
  value       = var.use_managed_node_group ? module.eks[0].cluster_name : aws_eks_cluster.jenkins_agent_eks_cluster[0].name
}