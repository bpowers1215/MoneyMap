# Money Map
An application that allows it’s users to easily and effectively manage their money.  The application will allow a user to add accounts which represent where their money is located, such as a checking account, savings account, interest bearing accounts, cash, etc.  The application is designed to be a representation of their total money map.  Users can enter all transactions with the level of detail they desire.  As a benefit, users will be able to create budgets and savings goals to keep spending on track.

_More effectively manage your money._

##Why Money Map?
+ Don’t hand over your financial account credentials. Yes, this puts more work on the user to sync all of their transactions, etc. but, the goal of Money Map is to make this extremely simple, easy, and secure.
+ No information sharing. Simply put, you own your data.  No need to worry about your information ever being shared with third parties, for any reason.
+ Multi-platform. The end goal is to have solutions across multiple devices.
+ Full control. Manage your money with the level of detail and in exactly the way that works for your. Customize your budget and transaction information to suit your needs.
+ Free. Managing your money shouldn’t cost you money.

##Features
1. User Management

   Allow multiple users to link to a single Checkbook account.  A primary user will be mandatory and constant.  Additional users can be added, edited, and removed.

2. Account Management

   Add, remove, and manage monetary accounts.

3. Transaction Category Management

   Add, remove, and manage transaction categories. Categories to be used for all transaction and budget data.

4. Transaction Management

   Add and edit transactions affecting monetary accounts. Report on transactions on an account basis.

5. Budget Management

   Add and edit budgets. Report on budget precision for budget term.

6. Bill Pay/Transaction Reminder

   Manage bill/transaction reminders that occur on a regular basis.

7. Savings Goals

   Adds ability to create savings goals linked to specific accounts.  Users can add available funds to savings goals and show progress toward the specified goal.

8. Additional Reports

   Add additional useful reports to money map management.

##Stack
###Deployment/Server Management
+ Docker - https://www.docker.com/

###Web API / Storage
+ Rust-Lang - https://www.rust-lang.org/
+ Nickel - http://nickel.rs/
+ MongoDB - https://www.mongodb.com/
+ MongoDB Drive for Rust - http://blog.mongodb.org/post/56426792420/introducing-the-mongodb-driver-for-the-rust

###Front-End
+ React - https://facebook.github.io/react/
+ Redux - http://redux.js.org/docs/basics/UsageWithReact.html

##Getting Started
###Starting application
```
#Build images for Docker containers
./bin/build

#Start application
./bin/start

#Restart application
./bin/restart

#Stop application (and remove containers)
./bin/stop
```
+ Access API on port 8080
+ Access DB on port 28017

##License
The Apache License Version 2.0
