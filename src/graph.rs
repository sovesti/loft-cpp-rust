use crate::json::JSONSerializer;
use crate::json::Bracket;

pub trait Serialize {
    fn serialize(&self, json: JSONSerializer) -> JSONSerializer;
}

pub struct Array<'a, T: Serialize> {
    name: String,
    members: [&'a T]
}

impl<T: Serialize> Serialize for Array<'_, T> {
    fn serialize(&self, mut json: JSONSerializer) -> JSONSerializer {
        if self.members.is_empty() {
            json.render_line(self.name.as_bytes(), b": []\n");
        } else {
            json.render_line(self.name.as_bytes(), b": [\n");
            json.prefix.expand();
            for member in &self.members {
                json = member.serialize(json);
            }
            json.prefix.shrink();
            json.render_bracket(Bracket::brace);
        }
        json
    }
}

enum Key {
    kind,
    name,
    display_name,
    _type,
    usr,
    signature,
    location,
    children,
}

impl Key {
    fn get_key(&self) -> &[u8] {
        match self {
            kind => b"kind",
            name => b"name",
            display_name => b"displayName",
            _type => b"type",
            usr => b"usr",
            signature => b"signature",
            location => b"location",
            children => b"children",
        }
    }
}

pub struct Node<'a> {
    kind: (Key, String),
    name: (Key, String),
    display_name: (Key, String),
    _type: (Key, String),
    usr: (Key, String),
    signature: (Key, String),
    location: (Key, String),
    children: Box<Array<'a, Node<'a>>>
}

impl Serialize for Node<'_> {
    fn serialize(&self, mut json: JSONSerializer) -> JSONSerializer {
        json.render_bracket(Bracket::l_curly);
        json.prefix.expand();
        for attr in [&self.kind, &self.name, &self.display_name, &self._type, &self.usr, &self.signature, &self.location] {
            json.render_line(attr.0.get_key(), attr.1.as_bytes());
        }
        json = self.children.serialize(json);
        json.prefix.shrink();
        json.render_bracket(Bracket::r_curly);
        json
    }
}