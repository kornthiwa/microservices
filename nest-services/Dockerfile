FROM node:16

WORKDIR /usr/src/nest-services

COPY . .

RUN npm install

CMD ["npm", "run", "start:prod"]

# Health check for Docker
HEALTHCHECK CMD curl --fail http://localhost:3000/health || exit 1
