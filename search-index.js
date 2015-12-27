var searchIndex = {};
searchIndex['urlparse'] = {"items":[[0,"","urlparse","This is a URL parsing library.",null,null],[3,"Url","","",null,null],[12,"scheme","","URL scheme specifier",0,null],[12,"netloc","","Network location part",0,null],[12,"path","","Hierarchical path",0,null],[12,"query","","Query component",0,null],[12,"fragment","","Fragment identifier",0,null],[12,"username","","User name",0,null],[12,"password","","Password",0,null],[12,"hostname","","Host name (lower case)",0,null],[12,"port","","Port number as integer",0,null],[5,"quote","","Replace special characters in string using the %xx escape.\nLetters, digits, and the characters '_.-' are never quoted.",null,null],[5,"quote_plus","","Like quote(), but also replace ' ' with '+', as required for quoting HTML form values.",null,null],[5,"unquote","","Replace %xx escapes by their single-character equivalent.",null,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[5,"unquote_plus","","Like unquote(), but also replace plus signs by spaces, as required for\nunquoting HTML form values.",null,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[5,"parse_qs","","Parse a query given as a string argument.",null,{"inputs":[{"name":"str"}],"output":{"name":"query"}}],[5,"urlparse","","Parse a URL and return `Url` object. This is synonymous with `Url::parse()`.",null,{"inputs":[{"name":"str"}],"output":{"name":"url"}}],[5,"urlunparse","","Return a URL string from `Url` object.\nThis is synonymous with `unparse()` defined in `Url`.",null,{"inputs":[{"name":"url"}],"output":{"name":"string"}}],[11,"cmp","","",0,{"inputs":[{"name":"url"},{"name":"url"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",0,{"inputs":[{"name":"url"},{"name":"url"}],"output":{"name":"option"}}],[11,"lt","","",0,{"inputs":[{"name":"url"},{"name":"url"}],"output":{"name":"bool"}}],[11,"le","","",0,{"inputs":[{"name":"url"},{"name":"url"}],"output":{"name":"bool"}}],[11,"gt","","",0,{"inputs":[{"name":"url"},{"name":"url"}],"output":{"name":"bool"}}],[11,"ge","","",0,{"inputs":[{"name":"url"},{"name":"url"}],"output":{"name":"bool"}}],[11,"hash","","",0,null],[11,"fmt","","",0,{"inputs":[{"name":"url"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"url"}],"output":{"name":"url"}}],[11,"eq","","",0,{"inputs":[{"name":"url"},{"name":"url"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"url"},{"name":"url"}],"output":{"name":"bool"}}],[11,"new","","Creates a new `Url` initialized with the empty string or None value.",0,{"inputs":[{"name":"url"}],"output":{"name":"url"}}],[11,"parse","","Parse a URL and return `Url` object.",0,{"inputs":[{"name":"url"},{"name":"str"}],"output":{"name":"url"}}],[11,"unparse","","Return a URL string from `Url` object.",0,{"inputs":[{"name":"url"}],"output":{"name":"string"}}],[11,"get_parsed_query","","Return a query object by executing `parse_qs()` with self.query.\nIf parsing a query fails, None value will be returned.",0,{"inputs":[{"name":"url"}],"output":{"name":"option"}}],[6,"Query","","An alias type of `HashMap<String, QueryValue>`.",null,null],[6,"QueryValue","","An alias type of `Vec<String>`.",null,null],[8,"GetQuery","","",null,null],[10,"get_first","","Get first value from Vec<String> via HashMap.get().",1,{"inputs":[{"name":"getquery"},{"name":"string"}],"output":{"name":"option"}}],[10,"get_from_str","","Get value from `Vec<String>` via `HashMap.get()`.\nThis requires one &str argument and returns `Option<QueryValue>`\ninstead of `Option<&QueryValue>`.",1,{"inputs":[{"name":"getquery"},{"name":"str"}],"output":{"name":"option"}}],[10,"get_first_from_str","","Get first value from `Vec<String>` via `HashMap.get()`.\nThis requires one &str argument and returns `Option<String>`\ninstead of `Option<&String>`.",1,{"inputs":[{"name":"getquery"},{"name":"str"}],"output":{"name":"option"}}],[11,"get_first","","",2,{"inputs":[{"name":"query"},{"name":"string"}],"output":{"name":"option"}}],[11,"get_from_str","","",2,{"inputs":[{"name":"query"},{"name":"str"}],"output":{"name":"option"}}],[11,"get_first_from_str","","",2,{"inputs":[{"name":"query"},{"name":"str"}],"output":{"name":"option"}}]],"paths":[[3,"Url"],[8,"GetQuery"],[6,"Query"]]};
initSearch(searchIndex);
