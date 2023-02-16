use crate::{to};

to!(
    should_not_add_extra_imports,
     r#"
       import { t, Plural } from "@lingui/macro";
       import { i18n } from "@lingui/core";
       import { Trans } from "@lingui/react";

       t`Test`;
       <Plural value={value} one="..." other="..."/>;
       <Trans>Untouched</Trans>
     "#,
    r#"
        import { i18n } from "@lingui/core";
        import { Trans } from "@lingui/react";

        i18n._({
          id: "NnH3pK",
          message: "Test"
        });

        <Trans id={"{value, plural, one {...} other {...}}"} values={{
          value: value
        }}/>;
        <Trans>Untouched</Trans>
    "#
);

to!(
    jsx_should_process_only_elements_imported_from_macro,
     r#"
      import { Plural } from "@lingui/macro";
      import { Select } from "./my-select-cmp";

      ;<Plural
       value={count}
       one="Message"
       other="Messages"
      />

      ;<Select prop="propValue">Should be untouched</Select>
     "#,

    r#"
       import { Trans } from "@lingui/react";
       import { Select } from "./my-select-cmp";

       ;<Trans
           id={"{count, plural, one {Message} other {Messages}}"}
           values={{ count: count }}
        />

      ;<Select prop="propValue">Should be untouched</Select>
    "#
);

to!(
    jsx_should_process_only_elements_imported_from_macro2,
     r#"
      import { Trans } from "@lingui/react";
      import { Plural } from "@lingui/macro";

      ;<Plural
       value={count}
       one="Message"
       other="Messages"
      />

      ;<Trans>Should be untouched</Trans>
     "#,

    r#"
       import { Trans } from "@lingui/react";

       ;<Trans
           id={"{count, plural, one {Message} other {Messages}}"}
           values={{ count: count }}
        />
       ;<Trans>Should be untouched</Trans>
    "#
);

to!(
    js_should_process_only_elements_imported_from_macro,
     r#"
      import { plural } from "@lingui/macro";
      import { t } from "./custom-t";

       t`Don't touch me!`
       plural(value, {one: "...", other: "..."})
     "#,

    r#"
       import { i18n } from "@lingui/core";
       import { t } from "./custom-t";

       t`Don't touch me!`
       i18n._({
          id: "kwTAtG",
          message: "{value, plural, one {...} other {...}}",
          values: {
              value: value
          }
       });

    "#
);

to!(
    js_should_process_only_elements_imported_from_macro2,
     r#"
      import { t } from "@lingui/macro";
      import { plural } from "./custom-plural";

       t`Hello World!`;
       plural(value, {one: "...", other: "..."});
     "#,

    r#"
       import { i18n } from "@lingui/core";
       import { plural } from "./custom-plural";

       i18n._({
          id: "0IkKj6",
          message: "Hello World!"
       });

       plural(value, {one: "...", other: "..."});
    "#
);


to!(
    js_should_support_renamed_imports,
     r#"
      import { t as i18nT, plural as i18nPlural } from "@lingui/macro";

       i18nT`Hello World!`;
       i18nPlural(value, {one: "...", other: "..."});
     "#,

    r#"
    import { i18n } from "@lingui/core";
    i18n._({
        id: "0IkKj6",
        message: "Hello World!"
    });
    i18n._({
        id: "kwTAtG",
        message: "{value, plural, one {...} other {...}}",
        values: {
            value: value
        }
    });
    "#
);
to!(
    jsx_should_support_renamed_imports,
     r#"
      import { Trans as I18nTrans, Plural as I18nPlural } from "@lingui/macro";

      ;<I18nPlural
       value={count}
       one="Message"
       other="Messages"
      />

      ;<I18nTrans>Hello!</I18nTrans>
     "#,

    r#"
        import { Trans } from "@lingui/react";

        ;<Trans id={"{count, plural, one {Message} other {Messages}}"} values={{
            count: count
        }}/>

        ;<Trans id={"Hello!"}/>
    "#
);
