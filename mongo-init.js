db = db.getSiblingDB('messenger');

db.users.drop();
db.users.createIndex({"name": 1}, {unique: true, background: true});

db.conversations.drop();
db.messages.drop();
