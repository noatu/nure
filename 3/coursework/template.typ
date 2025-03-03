
// Academic aliases {{{1

/// subject abbreviations to full names
#let subjects = (
  "БД": "Бази даних",
  "ОПНJ": "Основи програмування на Java",
  "ОС": "Операційні системи",
  "ПП": "Проектний практикум",
  "СПМ": "Скриптові мови програмування",
  "Ф": "Філософія",
)

/// education program abbreviations to name & number
#let edu_programs = (
  "ПЗПІ": (
    name: "Інженерія програмного забезпечення",
    number: 121, // TODO: ПЗПІ is "F2" now
  ),
)

// Template formatting functions {{{1

/// numberless heading
#let nheading(title) = heading(depth: 1, numbering: none, title)

/// fill horizontal space with a box and not an empty space
#let hfill(width) = box(width: width, repeat(" ")) // NOTE: This is a HAIR SPACE (U+200A), not a regular space

/// make underlined cell with filled value
#let uline(align: center, content) = underline[
  #if align != left { hfill(1fr) }
  #content
  #if align != right { hfill(1fr) }
]

/// bold text
#let bold(content) = text(weight: "bold")[#content]

/// month name from its number
#let month_gen(month) = (
  "січня",
  "лютого",
  "березня",
  "квітня",
  "травня",
  "червня",
  "липня",
  "серпня",
  "вересня",
  "жовтня",
  "листопада",
  "грудня",
).at(month - 1)

// Helper functions {{{1

/// captioned image with label derived from path:
/// - "image.png" = @image
/// - "img/image.png" = @image
/// - "img/foo/image.png" = @foo_image
/// - "img/foo/foo_image.png" = @foo_image
/// the caption will be modified based on a conditional positional value:
/// - `none`: no change
/// - some value: "`caption` (за даними `value`)"
/// - no value: "`caption` (рисунок виконано самостійно)"
/// additional named arguments will be passed to original `image` function
#let img(path, caption, ..sink) = {
  let parts = path.split(".").first().split("/")

  let label_string = if parts.len() <= 2 or parts.at(-1).starts-with(parts.at(-2)) {
    // ("image",), (_, "image") and (.., "img", "img_image")
    parts.last()
  } else {
    // (.., "img", "image") = "img_image"
    parts.at(-2) + "_" + parts.at(-1)
  }.replace(" ", "_")

  let caption = if sink.pos().len() == 0 {
    caption + " (рисунок виконано самостійно)"
  } else if sink.pos().first() == none {
    caption
  } else {
    [#caption (за даними #sink.pos().first())]
  }

  [#figure(image(path, ..sink.named()), caption: caption) #label(label_string)]
}

// Styling {{{1
/// NOTE: may be wrong
#let ua_alpha_numbering = "абвгдежиклмнпрстуфхцшщюя".split("") // 0 = "", 1 = "а"

// general outlook {{{2
// spacing between lines
#let spacing = 0.95em

#let style(it) = {
  set page(
    paper: "a4",
    margin: (top: 20mm, right: 10mm, bottom: 20mm, left: 25mm),
    number-align: top + right,
    numbering: (..numbers) => {
      if numbers.pos().at(0) != 1 {
        numbering("1", numbers.pos().at(0))
      }
    },
  )

  set text(font: ("Times New Roman", "Liberation Serif"), size: 14pt, hyphenate: false, lang: "uk")
  set par(justify: true, first-line-indent: (amount: 1.25cm, all: true))
  set underline(evade: false)

  // set 1.5 line spacing
  set block(spacing: spacing)
  set par(spacing: spacing)
  set par(leading: spacing)

  // enums and lists {{{2
  set enum(numbering: i => { ua_alpha_numbering.at(i) + ")" }, indent: 1.25cm, body-indent: 0.5cm)
  show enum: it => {
    set enum(indent: 0em, numbering: "1)")
    it
  }

  set list(indent: 1.35cm, body-indent: 0.5cm, marker: [--])

  // figures {{{2
  show figure: it => {
    v(spacing * 2, weak: true)
    it
    v(spacing * 2, weak: true)
  }

  set figure.caption(separator: [ -- ])
  show figure.where(kind: table): set figure.caption(position: top)
  show figure.caption.where(kind: table): set align(left)

  // figure numbering
  show heading.where(level: 1): it => {
    counter(math.equation).update(0)
    counter(figure.where(kind: image)).update(0)
    counter(figure.where(kind: table)).update(0)
    counter(figure.where(kind: raw)).update(0)
    it
  }
  set math.equation(numbering: (..num) => numbering("(1.1)", counter(heading).get().at(0), num.pos().first()))
  set figure(numbering: (..num) => numbering("1.1", counter(heading).get().at(0), num.pos().first()))

  // appearance of references to images and tables {{{2
  set ref(
    supplement: it => {
      if it == none or not it.has("kind") {
        it
      } else if it.kind == image {
        "див. рис."
      } else if it.kind == table {
        "див. таблицю"
      } else {
        it
      }
    },
  )
  show ref: it => {
    let el = it.element

    if el == none or not el.has("kind") {
      return it
    }
    if el.kind != image and el.kind != table {
      return it
    }

    [(#it)]
  }

  // headings {{{2
  set heading(numbering: "1.1")

  show heading.where(level: 1): it => {
    set align(center)
    set text(size: 14pt, weight: "semibold")

    pagebreak(weak: true)
    upper(it)
    v(spacing * 2, weak: true)
  }
  show heading.where(level: 2): it => {
    set text(size: 14pt, weight: "regular")

    v(spacing * 2, weak: true)
    block(width: 100%, spacing: 0em)[
      #h(1.25cm)
      #counter(heading).display(it.numbering)
      #it.body
    ]
    v(spacing * 2, weak: true)
  }

  show heading.where(level: 3): it => {
    set text(size: 14pt, weight: "regular")

    v(spacing * 2, weak: true)
    block(width: 100%, spacing: 0em)[
      #h(1.25cm)
      #counter(heading).display(it.numbering)
      #it.body
    ]
    v(spacing * 2, weak: true)
  }

  // listings {{{2
  show raw: it => {
    let new_spacing = 0.5em
    set block(spacing: new_spacing)
    set par(
      spacing: new_spacing,
      leading: new_spacing,
    )
    set text(size: 11pt, font: "Courier New", weight: "semibold")

    v(spacing * 2.5, weak: true)
    pad(it, left: 1.25cm)
    v(spacing * 2.5, weak: true)
  }

  it
}

// Coursework template {{{1

/// DSTU 3008:2015 Template for NURE
/// -> content
/// - doc (content): Content to apply the template to.
/// - title (str): Title of the document.
/// - subject_shorthand (str): Subject short name.
/// - department_gen (str): Department name in genitive form.
/// - authors ((name: str, full_name_gen: str, variant: int, group: str, gender: str),): List of Authors dicts.
/// - mentors ((name: str, gender: str, degree: str),): List of mentors dicts.
/// - edu_program_shorthand (str): Education program shorthand.
/// - task_list (done_date: datetime, initial_date: datetime, source: (content | str), content: (content | str), graphics: (content | str)): Task list object.
/// - calendar_plan ( plan_table: (content | str), approval_date: datetime): Calendar plan object.
/// - abstract (keywords: (str, ), text: (content | str)): Abstract object.
/// - bib_path path: Path to the bibliography yaml file.
/// - appendices (content): Content with appendices.
#let cw-template(
  doc,
  title: "NONE",
  subject_shorthand: "NONE",
  department_gen: "Програмної інженерії",
  author: (),
  mentors: (),
  edu_program_shorthand: "ПЗПІ",
  task_list: (),
  calendar_plan: (),
  abstract: (),
  bib_path: "bibl.yml",
  appendices: (),
) = {
  set document(title: title, author: author.name)

  show: style

  let bib-count = state("citation-counter", ())
  show cite: it => {
    it
    bib-count.update(((..c)) => (..c, it.key))
  }
  show bibliography: it => {
    set text(size: 0pt)
    it
  }


  let head_mentor = mentors.at(0)
  let edu_program = edu_programs.at(edu_program_shorthand)

  // page 1 {{{2
  [
    #set align(center)
    МІНІСТЕРСТВО ОСВІТИ І НАУКИ УКРАЇНИ

    ХАРКІВСЬКИЙ НАЦІОНАЛЬНИЙ УНІВЕРСИТЕТ РАДІОЕЛЕКТРОНІКИ

    \

    Кафедра Програмної інженерії

    \

    ПОЯСНЮВАЛЬНА ЗАПИСКА

    ДО КУРСОВОЇ РОБОТИ

    з дисципліни: "#subjects.at(subject_shorthand, default: "NONE")"

    Тема роботи: "#title"

    \ \ \

    #columns(2, gutter: 4cm)[
      #set align(left)

      #if author.gender == "m" { [Виконав\ ] } else { [Виконала\ ] } ст. гр. #author.group

      \
      Керівник:\
      #head_mentor.degree

      \
      Робота захищена на оцінку

      \
      Комісія:\
      #for mentor in mentors {
        [#mentor.degree\
        ]
      }

      #colbreak()
      #set align(left)

      \
      #author.name

      \ \
      #head_mentor.name

      \
      #underline(" " * 35)

      \ \
      #for mentor in mentors {
        [#mentor.name\
        ]
      }
    ]

    #v(1fr)

    Харків -- #task_list.done_date.display("[year]")

    #pagebreak()
  ]

  // page 2 {{{2
  {
    uline[Харківський національний університет радіоелектроніки]

    linebreak()
    linebreak()

    grid(
      columns: (100pt, 1fr),
      bold[
        Кафедра
        Дисципліна
        Спеціальність
      ],
      {
        uline(align: left, department_gen)
        linebreak()
        uline(align: left, subjects.at(subject_shorthand))
        linebreak()
        uline(align: left, [#edu_program.number #edu_program.name])
      },
    )
    grid(
      columns: (1fr, 1fr, 1fr),
      gutter: 0.3fr,
      [#bold[Курс] #uline(2)], [#bold[Група] #uline(author.group)], [#bold[Семестр] #uline(3)],
    )

    linebreak()
    linebreak()
    linebreak()

    align(center, bold[ЗАВДАННЯ \ на курсову роботу студента])

    linebreak()

    uline(align: left)[_#author.full_name_gen _]

    linebreak()
    linebreak()

    bold[\1. Тема роботи:]
    uline[#title.]

    linebreak()

    {
      bold[\2. Строк здачі закінченої роботи:]
      uline(task_list.done_date.display("[day].[month].[year]"))
      hfill(10fr)
    }

    linebreak()

    bold[\3. Вихідні дані для роботи:]
    uline(task_list.source)

    linebreak()

    bold[\4. Зміст розрахунково-пояснювальної записки:]
    uline(task_list.content)

    linebreak()

    bold[\5. Перелік графічного матеріалу:]
    uline(task_list.graphics)

    linebreak()

    {
      bold[\6. Дата видачі завдання:]
      uline(task_list.initial_date.display("[day].[month].[year]"))
      hfill(10fr)
    }

    pagebreak()
  }

  // page 3 {{{2
  {
    align(center, bold[КАЛЕНДАРНИЙ ПЛАН])
    set par(first-line-indent: 0pt)

    linebreak()

    calendar_plan.plan_table

    linebreak()

    grid(
      columns: (5fr, 5fr),
      grid(
        columns: (1fr, 2fr, 1fr),
        gutter: 0.2fr,
        [
          Студент \
          Керівник \
          #align(center)["#underline[#calendar_plan.approval_date.day()]"]
        ],
        [
          #uline(align: center, []) \
          #uline(align: center, []) \
          #uline(align: center, month_gen(calendar_plan.approval_date.month()))
        ],
        [
          \ \
          #underline[#calendar_plan.approval_date.year()] р.
        ],
      ),
      [
        #author.name, \
        #head_mentor.degree
        #head_mentor.name.
      ],
    )

    pagebreak()
  }

  // page 4 {{{2
  [
    #align(center, bold[РЕФЕРАТ]) \

    #context [
      #let pages = counter(page).final().at(0)
      #let images = query(figure.where(kind: image)).len()
      #let tables = query(figure.where(kind: table)).len()
      #let bibs = bib-count.final().dedup().len()
      /* TODO: why this stopped working?
      #let tables = counter(figure.where(kind: table)).final().at(0)
      #let images = counter(figure.where(kind: image)).final().at(0)*/

      #let counters = ()

      #if pages != 0 { counters.push[#pages с.] }
      #if tables != 0 { counters.push[#tables табл.] }
      #if images != 0 { counters.push[#images рис.] }
      #if bibs != 0 { counters.push[#bibs джерел] }

      Пояснювальна записка до курсової роботи: #counters.join(", ").
    ]

    \

    #{
      let keywords = abstract.keywords.map(upper)
      let is_cyrillic = word => word.split("").any(char => ("А" <= char and char <= "я"))

      let n = keywords.len()
      for i in range(n) {
        for j in range(0, n - i - 1) {
          if (
            (not is_cyrillic(keywords.at(j)) and is_cyrillic(keywords.at(j + 1)))
              or (
                is_cyrillic(keywords.at(j)) == is_cyrillic(keywords.at(j + 1)) and keywords.at(j) > keywords.at(j + 1)
              )
          ) {
            (keywords.at(j), keywords.at(j + 1)) = (keywords.at(j + 1), keywords.at(j))
          }
        }
      }

      keywords.join(", ")
    }

    \

    #abstract.text
  ]

  // page 5 {{{2
  outline(
    title: [
      ЗМІСТ
      #v(spacing * 2, weak: true)
    ],
    depth: 2,
    indent: auto,
  )

  doc

  // bibliography {{{2
  {
    heading(depth: 1, numbering: none)[Перелік джерел посилання]

    bibliography(
      bib_path,
      style: "ieee",
      full: true,
      title: none,
    )

    let bib_data = yaml(bib_path)

    let format-entry(citation) = {
      if (citation.type == "Web") {
        let date_array = citation.url.date.split("-")
        let date = datetime(
          year: int(date_array.at(0)),
          month: int(date_array.at(1)),
          day: int(date_array.at(2)),
        )
        [
          #citation.title.
          #citation.author.
          URL: #citation.url.value (дата звернення: #date.display("[day].[month].[year]")).
        ]
      } else if citation.type == "Book" [
        #citation.author
        #citation.title.
        #citation.publisher,
        #citation.date.
        #citation.page-total c.
      ] else [
        UNSUPPORTED BIBLIOGRAPHY ENTRY TYPE, PLEASE OPEN AN ISSUE
      ]
    }

    show enum.item: it => {
      set par(first-line-indent: 0pt)
      box(width: 1.25cm)
      box(width: 1em + 0.5cm)[#it.number.]
      it.body
      linebreak()
    }

    context {
      for (i, citation) in query(ref.where(element: none)).map(r => str(r.target)).dedup().enumerate() {
        enum.item(
          i + 1,
          format-entry(bib_data.at(citation)),
        )
      }
    }
  }

  // appendices {{{2
  {
    counter(heading).update(0)

    set heading(
      numbering: (i, ..nums) => {
        let char = upper(ua_alpha_numbering.at(i))
        if nums.pos().len() == 0 { char } else {
          char + "." + nums.pos().map(str).join(".")
        }
      },
    )

    show heading.where(level: 1): it => {
      set align(center)
      set text(size: 14pt, weight: "regular")

      pagebreak(weak: true)
      bold[ДОДАТОК #counter(heading).display(it.numbering)]
      linebreak()
      it.body
      v(spacing * 2, weak: true)
    }

    show heading.where(level: 2): it => {
      set text(size: 14pt, weight: "regular")

      v(spacing * 2, weak: true)
      block(width: 100%, spacing: 0em)[
        #h(1.25cm)
        #counter(heading).display(it.numbering)
        #it.body
      ]
      v(spacing * 2, weak: true)
    }

    appendices
  }
}

// Laboratory work template {{{1

/// DSTU 3008:2015 Template for NURE
/// -> content
/// - doc (content): Content to apply the template to.
/// - doctype ("ЛБ" | "ПЗ"): Document type.
/// - title (str): Title of the document.
/// - subject_shorthand (str): Subject short name.
/// - department_gen (str): Department name in genitive form.
/// - worknumber (int): Number of the work, can be omitted.
/// - authors ((name: str, full_name_gen: str, variant: int, group: str, gender: str),): List of Authors dicts.
/// - mentor (name: str, gender: str, degree: str): Mentors objects.
#let lab-pz-template(
  doc,
  doctype: "NONE",
  title: "NONE",
  subject_shorthand: "NONE",
  department_gen: "Програмної інженерії",
  worknumber: 1,
  authors: (),
  mentor: (),
) = {
  set document(title: title, author: authors.at(0).name)

  show: style

  context counter(heading).update(worknumber - 1)

  // page 1 {{{2
  align(center)[
    МІНІСТЕРСТВО ОСВІТИ І НАУКИ УКРАЇНИ \
    ХАРКІВСЬКИЙ НАЦІОНАЛЬНИЙ УНІВЕРСИТЕТ РАДІОЕЛЕКТРОНІКИ

    \ \
    Кафедра #department_gen

    \ \ \
    Звіт \
    з
    #if doctype == "ЛБ" [лабораторної роботи] else [практичної роботи]
    #if worknumber != none [№ #worknumber]

    з дисципліни: "#subjects.at(subject_shorthand, default: "UNLNOWN SUBJECT, PLEASE OPEN AN ISSUE")"

    з теми: "#title"

    \ \ \ \

    #columns(2)[
      #set align(left)
      #set par(first-line-indent: 0pt)
      #if authors.len() == 1 {
        let author = authors.at(0)
        if author.gender == "m" [Виконав:\ ] else [Виконала:\ ]
        [
          ст. гр. #author.group\
          #author.name\
        ]
        if author.variant != none [Варіант: №#author.variant]
      } else [
        Виконали:\
        ст. гр. #authors.at(0).group\
        #authors.map(a => [ #a.name\ ])
      ]

      #colbreak()
      #set align(right)

      #if mentor.gender == "m" { [Перевірив:\ ] } else { [Перевірила:\ ] }
      #mentor.degree #if mentor.degree.len() >= 15 [\ ]
      #mentor.name\
    ]

    #v(1fr)

    Харків -- #datetime.today().display("[year]")
  ]

  pagebreak(weak: true)

  heading(title)
  doc
}

// vim:sts=2:sw=2:fdl=0:fdm=marker:cms=/*%s*/
