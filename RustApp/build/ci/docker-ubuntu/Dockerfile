FROM public.ecr.aws/ubuntu/ubuntu:24.04_stable

RUN apt-get update

RUN apt-get install -y build-essential ca-certificates curl ebtables jq kmod liblzma-dev libssl-dev pkg-config make unzip
          
RUN install -m 0755 -d /etc/apt/keyrings
          
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc

RUN chmod a+r /etc/apt/keyrings/docker.asc
          
RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
          
RUN apt-get update
          
RUN export TZ=Asia/Jakarta
          
RUN export DEBIAN_FRONTEND=noninteractive
          
RUN apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
          
RUN systemctl enable docker
          
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-aarch64.zip" -o "awscliv2.zip"
          
RUN unzip awscliv2.zip
          
RUN ./aws/install --bin-dir /usr/local/bin --install-dir /usr/local/aws-cli --update
          
RUN rm -rf awscliv2
          
RUN rm -rf awscliv2.zip