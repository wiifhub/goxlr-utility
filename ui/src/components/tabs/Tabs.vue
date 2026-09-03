<template>
  <div class="tabs" :class="`tabs--${variant}`">
    <div class="tab" role="TabList" :aria-label="tabListLabel">
      <button v-for="tab in tabs" :key="tab.name"
        :class="{ active: tab.isActive }" v-show="!tab.hidden" @click="selectTab(tab)" role="tab"
        :aria-selected="tab.isActive" :tabindex="tab.isActive ? 0 : -1" @keydown="onTabKeydown" :ref="tab.name">
        {{ tab.name }}
      </button>
    </div>
    <div class="tabs-details" role="tabpanel" :aria-label="getActiveTab().name">
      <slot></slot>
    </div>
  </div>
</template>

<script>
export default {
  emits: ["on-change"],
  name: "TabList",

  data() {
    return { tabs: [] };
  },
  computed: {
    tabListLabel() {
      return this.label || this.$t('message.common.tabList');
    },
  },
  props: {
    variant: {
      type: String,
      required: false,
      default: "standard",
    },
    label: {
      type: String,
      required: false,
      default: "Tab list",
    },
  },

  created() {
    window.addEventListener("keydown", this.onTabKeydownGlobal)
  },
  unmounted() {
    window.removeEventListener("keydown", this.onTabKeydownGlobal)
  },
  methods: {
    selectTab(selectedTab) {
      let activeTab = this.tabs.find((tab) => tab.isActive);
      this.tabs.forEach((tab) => {
        tab.isActive = tab.id === selectedTab.id;
      });
      let newActive = this.tabs.find((tab) => tab.isActive);

      if (activeTab !== newActive) {
        // Make sure we mount the tab before we call an update..
        this.$nextTick(() => this.$emit("on-change", selectedTab));
      }
    },

    // This function is generally for external calls, to set directly by id.
    selectTabById(id) {
      let tab = this.tabs.find((tab) => tab.id === id);
      this.selectTab(tab);
    },

    getActiveTab() {
      //return the active tab
      const activeTab = this.tabs.find((tab) => tab.isActive);
      if (activeTab) {
        return activeTab;
      } else {
        return "";
      }
    },
    //keyboard navigation
    onTabKeydown(event) {
      const tabs = this.tabs;
      const activeTab = this.getActiveTab();
      const activeTabIndex = tabs.indexOf(activeTab);
      let nextTab;
      switch (event.key) {
        case "ArrowRight":
        case "ArrowDown":
        case "PageDown":
          nextTab = tabs[(activeTabIndex + 1) % tabs.length];
          break;
        case "ArrowLeft":
        case "ArrowUp":
        case "PageUp":
          nextTab = tabs[(activeTabIndex - 1 + tabs.length) % tabs.length];
          break;
        case "Home":
          nextTab = tabs[0];
          break;
        case "End":
          nextTab = tabs[tabs.length - 1];
          break;
        default:
          break;
      }


      if (nextTab) {
        this.selectTab(nextTab);
        //nextTab.$el is the button element
        //we need a ref on the button element to focus it
        this.$refs[nextTab.name][0].focus();
      }
    },
    onTabKeydownGlobal(event) {
      if (this.label !== "Device Settings") return;
      const tabs = this.tabs;
      // const activeTab = this.getActiveTab();
      // const activeTabIndex = tabs.indexOf(activeTab);
      let nextTab;
      if (event.shiftKey && event.ctrlKey) {
        // Shift(Number) have different symbol between US keyboard and Other language.
        switch (event.code) {
          case "Digit1":
          case "Digit2":
          case "Digit3":
          case "Digit4":
          case "Digit5":
          case "Digit6":
          case "Digit7":
          case "Digit8":
            nextTab = tabs[Number(event.code[5]) - 1];
            break;
          default:
            break;
        }
      }

      if (nextTab) {
        this.selectTab(nextTab);
        //nextTab.$el is the button element
        //we need a ref on the button element to focus it
        this.$refs[nextTab.name][0].focus();
      }
    },
  },
  mounted() {
    this.$emit("on-change", this.getActiveTab());
  }
};
</script>

<style>
.tabs {
  min-width: 0;
}

.tab {
  border-bottom: 1px solid #59b1b6;
  text-align: left;
}

.tabs--workspace {
  display: grid;
  grid-template-columns: 184px minmax(0, 1fr);
  min-height: 470px;
}

.tabs--workspace > .tab {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px;
  border: 0;
  border-right: 1px solid var(--border-subtle);
  background: var(--surface-sidebar);
}

.tabs--workspace > .tab button {
  min-width: 0;
  max-width: none;
  min-height: 44px;
  padding: 0 14px;
  margin: 0;
  border: 1px solid transparent;
  border-radius: var(--radius-control);
  text-align: left;
  color: var(--text-secondary);
  transition: color var(--motion-fast), background-color var(--motion-fast), border-color var(--motion-fast), transform var(--motion-fast);
}

.tabs--workspace > .tab button:hover:not(.active) {
  color: var(--text-primary);
  background: var(--surface-hover);
}

.tabs--workspace > .tab button.active {
  color: var(--accent);
  background: var(--accent-muted);
  border: 1px solid var(--accent-border);
  border-bottom-color: var(--accent-border);
  text-shadow: none;
}

.tabs--workspace > .tab button.active::before {
  content: "";
  display: inline-block;
  width: 6px;
  height: 6px;
  margin-right: 9px;
  border-radius: 50%;
  background: var(--accent);
  box-shadow: 0 0 12px var(--accent);
}

.tabs--workspace > .tabs-details {
  min-width: 0;
  border: 0;
  overflow: auto;
  background: var(--surface-content);
}

.tabs--compact > .tab button {
  min-width: 96px;
  min-height: 40px;
  padding: 8px 14px;
}

@media (max-width: 900px) {
  .tabs--workspace {
    display: block;
  }

  .tabs--workspace > .tab {
    position: sticky;
    top: 0;
    z-index: 5;
    display: flex;
    flex-direction: row;
    overflow-x: auto;
    padding: 8px;
    border-right: 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .tabs--workspace > .tab button {
    flex: 0 0 auto;
    text-align: center;
  }
}

.tab button {
  background-color: inherit;
  border: none;
  outline: none;
  cursor: pointer;
  padding: 10px 20px;
  margin-bottom: -1px;
  min-width: 150px;
  max-width: min-content;

  /*font-family: LeagueMonoVariable, sans-serif;*/
  border-radius: 5px 5px 0 0;
  color: #fff;
  white-space: nowrap;
}

.tab button:hover:not(.active) {
  background-color: #2d3230;
}

.tab button.active {
  border: 1px solid #59b1b6;
  border-bottom: 1px solid #252927;

  text-shadow: 0 0 3px #59b1b6, 0 0 5px #59b1b6;
}

.tabs-details {
  border: 1px solid #59b1b6;
  border-top: 0;
  padding: 0;
  margin: 0;
  overflow: auto;
  vertical-align: middle;
}
</style>
