/*
Language: JavaScript
Description: JavaScript (JS) is a lightweight, interpreted, or just-in-time compiled programming language with first-class functions.
Category: common, scripting
Website: https://developer.mozilla.org/en-US/docs/Web/JavaScript
*/

var ATTRIBUTES = {
  begin: '(.*?):',
  end: '$',
  className: 'attribute',
  returnBegin: true,
  excludeEnd: true,
  contains: [
    {
      begin: '(.*?):',
      className: 'attribute-name'
    },
    {
      className: 'attribute-value',
      endsWithParent: true
    }
  ]
};

var MACRO_INSERT = {
  begin: '\\${',
  end: '}',
  className: 'macro-insert'
};

var EVENT_LINE = {
  endsWithParent: true,
  className: 'event-line',
  contains: [{
    begin: '\\[',
    end: '\\]',
    className: 'event',
    contains: [
      MACRO_INSERT
    ]
  },
    MACRO_INSERT
  ]
};

var COMMENT_LINE = {
  begin: '//(.*)$',
  className: 'comment'
};

window.hljsjavascript = function(hljs) {
  return {
    aliases: ['ssb'],
    contains: [
      {
        begin: '(?=#INFO)',
        end: '(?![^#])',
        className: 'section',
        contains: [
          COMMENT_LINE,
          {
            begin: '#INFO',
            className: 'section-title'
          },
          ATTRIBUTES
        ]
      },
      {
        begin: '(?=#TARGET)',
        end: '(?![^#])',
        className: 'section',
        contains: [
          COMMENT_LINE,
          {
            begin: '#TARGET',
            className: 'section-title'
          },
          ATTRIBUTES
        ]
      },
      {
        begin: '(?=#MACROS)',
        end: '(?![^#])',
        className: 'section',
        contains: [
          COMMENT_LINE,
          {
            begin: '#MACROS',
            className: 'section-title'
          },
          {
            begin: '(.*?):',
            end: '$',
            className: 'attribute',
            returnBegin: true,
            excludeEnd: true,
            contains: [
              {
                begin: '^(.*?):',
                className: 'macro-name'
              },
              // this matches everything not matched yet but would match everything until the end of the document, so we need endsWithParent:true
              {
                className: 'macro-value',
                endsWithParent: true,
                contains: [EVENT_LINE]
              }
            ]
          }
        ]
      },
      {
        begin: '(?=#EVENTS)',
        end: '(?![^#])',
        className: 'section',
        contains: [
          COMMENT_LINE,
          {
            begin: '#EVENTS',
            className: 'section-title'
          },
          {
            begin: '^[^/][^/](.*?)[|](.*?)[|](.*?)[|](.*)',
            end: '$',
            className: 'event',
            returnBegin: true,
            returnEnd: true,
            contains: [
              {
                begin: '^(.*?)(?=[|](.*?)[|](.*?)[|](.*))',
                className: 'event-timestamp'
              },
              {
                begin: '[|]',
                className: 'event-separator'
              },
              {
                begin: '(.*?)(?=[|](.*?)[|](.*))',
                className: 'event-macro'
              },
              {
                begin: '(.*?)(?=[|](.*))',
                className: 'comment'
              },
              {
                className: 'event-text',
                contains: [
                  EVENT_LINE
                ],
                endsWithParent: true
              }
            ]
          }
        ]
      },
      {
        begin: '(?=#RESOURCES)',
        end: '(?![^#])',
        className: 'section',
        contains: [
          COMMENT_LINE,
          {
            begin: '#RESOURCES',
            className: 'section-title'
          },
          ATTRIBUTES
        ]
      }
    ]
  };
};
