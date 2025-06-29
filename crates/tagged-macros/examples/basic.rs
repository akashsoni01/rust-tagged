use tagged_core::*;
use tagged_macros::Tagged;

#[derive(Tagged)]
struct EmployeeId(i32);

struct Employee {
    id: EmployeeId,
    employee_email_id: Tagged<String, Self>,
    name: String,
    org: Org,
}

struct Org {
    org_email_id: Tagged<String, Self>,
    name: String,
}

fn send_mail_employee(mail_id: &Tagged<String, crate::Employee>, message: &str) {
    send_mail(mail_id, message);
}

fn send_mail_org(mail_id: &Tagged<String, crate::Org>, message: &str) {
    send_mail(mail_id, message);
}

fn send_mail(mail_id: &str, message: &str) {
    println!("Mail Sent.{}", message);
}


fn main() {
    let emp = Employee {
        id: 12.into(),
        employee_email_id: "akash@gmail.com".into(),
        name: "Akash".into(),
        org: Org {
            org_email_id: "info@codefonsi.com".into(),
            name: "Codefonsi".into(),
        },
    };

    // here we can clearly define and distinct the mail id of employee and org
    // without
    // // expected `&Tagged<String, Org>`, but found `&Tagged<String, Employee>`
    // send_mail_org(&emp.employee_email_id, "This is supposed to send to user but there is no type safety at compile time");
    // 
    // // expected `&Tagged<String, Employee>`, but found `&Tagged<String, Org>`
    // send_mail_employee(&emp.org.org_email_id, "This is supposed to send to user but there is no type safety at compile time");
    // 
    // // the trait bound `Tagged<String, Employee>: From<Tagged<String, Org>>` is not satisfied [E0277]
    // send_mail_employee(&emp.org.org_email_id.into(), "This is ok");

    // after refactoring
    send_mail_org(&emp.org.org_email_id, "This is ok");
    send_mail_employee(&emp.employee_email_id, "This is ok");


}