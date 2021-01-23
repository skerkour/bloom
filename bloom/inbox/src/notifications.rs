use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsletterEmailParams {
    pub content: String,
    pub unsubscribe_link: Option<String>,
}

pub const NEWSLETTER_EMAIL_TEMPLATE_ID: &str = "NEWSLETTER_EMAIL_TEMPLATE";
pub const NEWSLETTER_EMAIL_TEMPLATE: &str = r##"
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
<html lang="en">
<head>
  <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">

  <title></title>

  <style type="text/css">
  </style>
</head>
<body style="margin:0; padding:0; background-color:#ffffff; font-size:16px;">
  <center>
    <table width="600" border="0" cellpadding="0" cellspacing="0" bgcolor="#ffffff" align="left">
        <tr>
            <td align="left" valign="top">
            {{ content }}


            {% if unsubscribe_link %}
            <div class="unsubscribe">
                <br /><br /><br />
                <a href="{{ unsubscribe_link }}">Unsubscribe</a>
                <br />
           </div>
           {% endif %}

            </td>
        </tr>
    </table>
  </center>
</body>
</html>
"##;
