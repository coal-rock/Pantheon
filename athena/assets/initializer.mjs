export default function initializer() {
  return {
    onStart: () => {
      console.log("Loading...");
      console.time("trunk-initializer");

      document.body.insertAdjacentHTML("afterbegin", `
<div class="pf-v5-l-bullseye">
  <div class="pf-v5-l-bullseye__item" style="width: 50%;">
    <div id="initializer" class="pf-v5-c-progress">
      <div
        class="pf-v5-c-progress__description"
        id="initializerState"
      >Loading</div>
      <div id="initializerStatus" class="pf-v5-c-progress__status" aria-hidden="true">
        <span id="initializerLabel" class="pf-v5-c-progress__measure">0%</span>
      </div>
      <div
        class="pf-v5-c-progress__bar"
        id="initializerBar"
        role="progressbar"
        aria-valuemin="0"
        aria-valuemax="100"
        aria-valuenow="0"
        aria-labelledby="progress-simple-example-description"
      >
        <div id="initializerValue" class="pf-v5-c-progress__indicator" style="width:0;"></div>
      </div>
    </div>
    <div id="initializerHelper" class="pf-v5-c-progress__helper-text pf-m-hidden">
      <div class="pf-v5-c-helper-text">
        <div class="pf-v5-c-helper-text__item">
          <span id="initializerHelperText" class="pf-v5-c-helper-text__item-text"></span>
        </div>
      </div>
    </div>
  </div>
</div>
`);

    },
    onProgress: ({current, total}) => {
      console.debug("current:", current, "total:", total);

      if (!total) {
        console.log("Loading...", current, "bytes");
      } else {
        const value = Math.round((current / total) * 100)
        console.log("Loading...", value, "%")
        setProgress(value);
      }
    },
    onComplete: () => {
      console.log("Loading... done!");
      console.timeEnd("trunk-initializer");
    },
    onSuccess: (_wasm) => {
      console.log("Loading... successful!");
      setProgress(100);
      setState("Complete");
    },
    onFailure: (error) => {
      console.error("Loading... failed!", error);
      setState("Failed", error);
    }
  }
};

function setProgress(value) {
  const label = document.getElementById("initializerLabel");
  if (label) {
    label.innerText = `${value}%`;
  }
  const bar = document.getElementById("initializerBar");
  if (bar) {
    bar.ariaValueNow = value;
  }
  const width = document.getElementById("initializerValue");
  if (width) {
    width.style.width = `${value}%`;
  }
}

function setState(value, helperText) {
  const state = document.getElementById("initializerState");
  if (state) {
    state.innerText = value;
  }
  if (value === "Failed") {
    const component = document.getElementById("initializer");
    if (component) {
      component.classList.add("pf-m-danger");
    }
    const status = document.getElementById("initializerStatus");
    status.innerHTML = `
      <span class="pf-v5-c-progress__status-icon">
        <i class="fas fa-fw fa-times-circle" aria-hidden="true"></i>
      </span>
    `;
  }

  const helper = document.getElementById("initializerHelper");
  if (helper) {
    if (helperText === undefined) {
      helper.classList.add("pf-m-hidden");
    } else {
      helper.classList.remove("pf-m-hidden");
      const helperContent = document.getElementById("initializerHelperText");
      if (helperContent) {
        helperContent.innerText = helperText;
      }
    }
  }

}
