struct Employee {
    employee_email_id: String,
    name: String,
    org: Org,
}

struct Org {
    org_email_id: String,
    name: String,
}

fn send_mail_employee(mail_id: &str, message: &str) {
    send_mail(mail_id, message);
}

fn send_mail_org(mail_id: &str, message: &str) {
    send_mail(mail_id, message);
}

fn send_mail(mail_id: &str, message: &str) {
    println!("Mail Sent.{}", message);
}


fn main() {
    let emp = Employee {
    employee_email_id: "akash@gmail.com".into(),
    name: "Akash".into(),
    org: Org {
            org_email_id: "info@codefonsi.com".into(),
            name: "Codefonsi".into(),
    },
    };

    send_mail_org(&emp.employee_email_id, "This is supposed to send to user but there is no type safety at compile time");
    send_mail_employee(&emp.org.org_email_id, "This is supposed to send to user but there is no type safety at compile time");

    // after refactoring
    send_mail_org(&emp.org.org_email_id, "This is ok");
    send_mail_employee(&emp.employee_email_id, "This is ok");


}