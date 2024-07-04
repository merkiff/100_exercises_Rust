// TODO: Use two variants, one for a title error and one for a description error.
//   Each variant should contain a string with the explanation of what went wrong exactly.
//   You'll have to update the implementation of `Ticket::new` as well.
#[derive(Debug)]
enum TicketNewError {
    TitleErr(String),
    DescriptionErr(String),
}

// TODO: `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".

// Ticket::new이 성공하면 Ok(ticket) 패턴으로 매칭되어 ticket을 반환합니다.
// Ticket::new이 실패하여 Err(TicketNewError::DescriptionErr(_))가 반환되면, 기본 설명("Description not provided")을 사용하여 다시 Ticket::new을 호출하고 unwrap으로 성공적인 결과를 강제합니다.
// Ticket::new이 실패하여 Err(TicketNewError::TitleErr(error))가 반환되면, panic!을 호출하여 프로그램을 종료합니다. 오류 메시지는 error 변수에 저장된 메시지를 사용합니다.
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(ticket) => ticket,
        Err(TicketNewError::DescriptionErr(_)) => {
            Ticket::new(title, "Description not provided".to_string(), status).unwrap()
        }
        Err(TicketNewError::TitleErr(error)) => panic!("{error}"),
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleErr(
                "Title cannot be empty".to_string(),
            ));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleErr(
                "Title cannot be longer than 50 bytes".to_string(),
            ));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionErr(
                "Description cannot be empty".to_string(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionErr(
                "Description cannot be longer than 500 bytes".to_string(),
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
