FROM node:21-alpine3.17

WORKDIR /code
COPY . /code
RUN chmod +x /code/setup.sh

RUN npm install
CMD ["npm", "run", "migration"]