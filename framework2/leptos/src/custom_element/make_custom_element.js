export function make_custom_element(
  superclass,
  tag_name,
  shadow,
  constructor,
  observedAttributes,
  observedProperties,
  superclassTag
) {
  customElements.define(
    tag_name,
    class extends superclass {
      static get observedAttributes() {
        return observedAttributes;
      }

      constructor() {
        super();

        // run whatever custom constructor we've been given, and other setup as necessary
        constructor(this);

        if (shadow && !this.shadowRoot) {
          this.attachShadow({ mode: "open" });
        }

        // define setters/getters for properties
        for (const propertyName of observedProperties) {
          Object.defineProperty(this, propertyName, {
            set(value) {
              console.log("trying to set", propertyName, "to", value);
              return this._setProperty(propertyName, value);
            }
          });
        }
      }

      attributeChangedCallback(name, oldValue, newValue) {
        console.log(this.tagName, name, oldValue, newValue);
        this._attributeChangedCallback(this, name, oldValue || "", newValue);
      }

      connectedCallback() {
        this._injectChildren(this, this.dataset.leptosHydrate == "");
      }
    },
    superclassTag ? { extends: superclassTag } : undefined
  );
}