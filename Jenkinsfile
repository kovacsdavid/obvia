pipeline {
    agent any

    environment {
        DOCKER_REGISTRY = credentials('docker-registry')
        VERSION = sh(script: 'git describe --tags --always', returnStdout: true).trim()
        VITE_GIT_COMMIT_HASH = "${VERSION}"
        DEPLOY_SERVER = credentials('deploy-server')
        DEPLOY_SSH = credentials('deploy-ssh-key')
        BRANCH_NAME = "${GIT_BRANCH.split("/")[1]}"
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Backend Tests') {
            steps {
                dir('backend') {
                    sh 'cargo test'
                    sh 'cargo clippy -- -D warnings'
                }
            }
        }

        stage('Frontend Tests') {
            steps {
                dir('frontend') {
                    sh 'npm install'
                    sh 'npx vitest'
                    sh 'npm run lint'
                }
            }
        }

        stage('Build & Push') {
            steps {
                script {
                    docker.withRegistry("http://${DOCKER_REGISTRY}") {
                        def backend_build = docker.build(
                            "${DOCKER_REGISTRY}/backend-${BRANCH_NAME}:${VERSION}",
                             "-t ${DOCKER_REGISTRY}/backend-${BRANCH_NAME}:latest ./backend"
                        )
                        def frontend_build = docker.build(
                            "${DOCKER_REGISTRY}/frontend-${BRANCH_NAME}:${VERSION}",
                             "-t ${DOCKER_REGISTRY}/frontend-${BRANCH_NAME}:latest --build-arg VITE_GIT_COMMIT_HASH=${VITE_GIT_COMMIT_HASH} ./frontend"
                        )

                        backend_build.push()
                        frontend_build.push()
                        backend_build.push('latest')
                        frontend_build.push('latest')
                    }
                }
            }
        }

        stage('Deploy') {
            steps {
                script {
                    sshagent(['deploy-ssh-key']) {
                        sh """
                            ssh -o StrictHostKeyChecking=no \${DEPLOY_SSH_USR}@\${DEPLOY_SERVER} '
                                sudo /opt/deployment/deploy.sh \
                                ${DOCKER_REGISTRY} \
                                ${VERSION} \
                                ${BRANCH_NAME}
                            '
                        """
                    }
                }
            }
        }
    }

    post {
        always {
            cleanWs()
        }
        success {
            echo 'Pipeline succeeded!'
        }
        failure {
            echo 'Pipeline failed!'
        }
    }
}
