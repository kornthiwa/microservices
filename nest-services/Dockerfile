FROM node:22

WORKDIR /usr/src/nest-services

COPY . .

RUN npm install

RUN npm run build

CMD ["npm", "run", "start:prod"]

# Health check for Docker
HEALTHCHECK CMD curl --fail http://localhost:3000/health || exit 1
