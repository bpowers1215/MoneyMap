//init_db.js
/**
*   Initialize the database
*   Set-up db users/priveleges
*/
load('/db_config/db_properties.js');

conn = new Mongo();

print("Getting admin DB...");
adminDB = conn.getDB("admin");

print("Creating admin user");
adminDB.createUser({user: MM_DB_ADMIN_USER, pwd: MM_DB_ADMIN_PASS, roles:[ { role: "root", db: "admin" }, 'userAdminAnyDatabase', 'readWriteAnyDatabase']});

print("Authenticating admin user...");
adminDB.auth(MM_DB_ADMIN_USER, MM_DB_ADMIN_PASS);

print("Creating application database (by inserting temp data)...")
appDB = conn.getDB(MM_DB_NAME);
appDB.users.insert({"_id": 1, "value":"temp data"});

print("Creating application/client user...");
appDB.createUser({user: MM_DB_CLIENT_USER, pwd: MM_DB_CLIENT_PASS, roles:['readWrite']});

print("Removing temp data from database creation...")
appDB.users.remove({"_id": 1});
