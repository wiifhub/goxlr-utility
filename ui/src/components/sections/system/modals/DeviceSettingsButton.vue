<template>
  <div>
    <BigButton
        id="device_settings_button"
        ref="device_button"
        :title="$t('message.system.deviceButton')"
        @button-clicked="$refs.modal.openModal(undefined, $refs.device_button)"
    >
      <font-awesome-icon icon="fa-solid fa-headphones"/>
    </BigButton>

    <AccessibleModal
        width="630px"
        ref="modal"
        id="device_settings"
        :show_footer="false"
    >
      <template v-slot:title>{{ $t('message.system.deviceButton') }}</template>

      <div class="settingList" role="region" :aria-label="$t('message.system.deviceButton')">
        <ListSetting v-if="isDeviceMini() || (!isDeviceMini() && firmwareSupportsMix2())" :value="getVodMode()" :options="getVodModeKeys()"
                     :description="`Sets ${getKeyForMix2()} behaviour`"
                     :label="`${getKeyForMix2()} Behaviour`" @change="setVodMode"/>

        <NumberSetting :value="getHold()" :min="0" :max="5000" suffix="ms" @change="updateHold"
                       :label="$t('message.system.device.holdDuration')"
                       :description="$t('message.system.device.holdDurationAccessibility')"/>

        <NumberSetting v-if="!isDeviceMini()" :value="getSamplerPreRecord()" :min="0" :max="30" suffix="s"
                       @change="updateSamplerPreRecord" :label="$t('message.system.device.sampleBuffer')"
                       :description="$t('message.system.device.sampleBufferAccessibility')"/>

        <NumberSetting v-if="!isDeviceMini()" :value="getSamplerFadeDuration()" :min="0" :max="20000" suffix="ms"
                       @change="updateSamplerFadeDuration" :label="$t('message.system.device.samplerFadeDuration')"
                       :description="$t('message.system.device.samplerFadeDurationAccessibility')"/>

        <BooleanSetting :label="$t('message.system.device.voiceDeafen')" :enabled="get_vcmaammtcm()"
                        @change="set_vcmaammtcm" :description="$t('message.system.device.voiceDeafenAccessibility')"/>

        <BooleanSetting v-if="!isDeviceMini()" :label="$t('message.system.device.monitorWithFx')"
                        :enabled="get_mic_monitor_with_fx()" @change="set_mic_monitor_with_fx"
                        :description="$t('message.system.device.monitorWithFxAccessibility')"/>
        <BooleanSetting v-if="!isDeviceMini()" :label="$t('message.system.device.resetSampleFunctionOnClear')"
                        :enabled="get_reset_sample_function()" @change="set_reset_sample_function"
                        :description="$t('message.system.device.resetSampleFunctionOnClearAccessibility')"/>
        <BooleanSetting v-if="!isDeviceMini()" :label="$t('message.system.device.lockFaders')"
                        :enabled="get_locked_faders()" @change="set_locked_faders"
                        :description="$t('message.system.device.lockFadersAccessibility')"/>

      </div>

    </AccessibleModal>

  </div>
</template>

<script>
import AccessibleModal from "@/components/design/modal/AccessibleModal.vue";
import SimpleNumberInput from "@/components/design/SimpleNumberInput.vue";
import {store} from "@/store";
import {websocket} from "@/util/sockets";
import BigButton from "@/components/buttons/BigButton.vue";
import {
  driverMix2,
  driverPreVOD,
  driverVOD,
  firmwareSupportsMix2,
  isDeviceMini,
  isStreamNoMusic,
  isWindowsDriver,
  versionNewerOrEqualTo
} from "@/util/util";
import SettingsButton from "@/components/sections/system/modals/SettingsButton.vue";
import BooleanSetting from "@/components/sections/system/modals/settings/BooleanSetting.vue";
import NumberSetting from "@/components/sections/system/modals/settings/NumberSetting.vue";
import ListSetting from "@/components/sections/system/modals/settings/ListSetting.vue";

export default {
  name: "DeviceSettingsButton",
  components: {
    ListSetting,
    NumberSetting,
    BooleanSetting,
    SettingsButton,
    BigButton,
    SimpleNumberInput,
    AccessibleModal
  },

  methods: {
    firmwareSupportsMix2,
    isDeviceMini,
    getHold() {
      return store.getActiveDevice().settings.mute_hold_duration;
    },

    updateHold(value) {
      websocket.send_command(store.getActiveSerial(), {SetMuteHoldDuration: value});
    },

    getSamplerPreRecord() {
      return Math.ceil(store.getActiveDevice().sampler.record_buffer / 1000);
    },

    updateSamplerPreRecord(seconds) {
      websocket.send_command(store.getActiveSerial(), {"SetSamplerPreBufferDuration": seconds * 1000});
    },

    getSamplerFadeDuration() {
      return Math.ceil(store.getActiveDevice().settings.fade_duration);
    },

    updateSamplerFadeDuration(millis) {
      websocket.send_command(store.getActiveSerial(), {"SetSamplerFadeDuration": millis});
    },

    get_vcmaammtcm() {
      // I hate this name :D
      if (!store.getActiveDevice()) {
        return false;
      }
      return store.getActiveDevice().settings.vc_mute_also_mute_cm;
    },

    set_vcmaammtcm(value) {
      websocket.send_command(store.getActiveSerial(), {"SetVCMuteAlsoMuteCM": value});
    },

    get_mic_monitor_with_fx() {
      if (!store.getActiveDevice()) {
        return false;
      }
      return store.getActiveDevice().settings.enable_monitor_with_fx;
    },

    set_mic_monitor_with_fx(value) {
      websocket.send_command(store.getActiveSerial(), {"SetMonitorWithFx": value});
    },

    get_reset_sample_function() {
      if (!store.getActiveDevice()) {
        return false;
      }
      return store.getActiveDevice().settings.reset_sampler_on_clear;
    },

    set_reset_sample_function(value) {
      websocket.send_command(store.getActiveSerial(), {"SetSamplerResetOnClear": value});
    },

    get_locked_faders() {
      if (!store.getActiveDevice()) {
        return false;
      }
      return store.getActiveDevice().settings.lock_faders;
    },

    set_locked_faders(value) {
      websocket.send_command(store.getActiveSerial(), {"SetLockFaders": value});
    },

    getVodModeKeys() {
      return [
        {
          key: "Routable",
          value: "Routable",
        },
        {
          key: "StreamNoMusic",
          value: "Stream Mix (No Music)",
        },
      ];
    },

    getVodMode() {
      if (isStreamNoMusic()) {
        return "StreamNoMusic";
      }
      return "Routable";
    },

    setVodMode(value) {
      websocket.send_command(store.getActiveSerial(),{"SetVodMode": value});
    },

    getKeyForMix2() {
      let sample = "Sampler";
      let vod = "VOD";
      let mix2 = "Stream Mix 2";

      if (store.hasActiveDevice()) {
        if (isWindowsDriver()) {
          if (driverPreVOD()) {
            return sample
          } else if (driverVOD()) {
            return vod
          } else if (driverMix2()) {
            return mix2
          }
        } else {
          // We're on Linux, so there's no reliable way to check the alsa-ucm-conf package, if they
          // have the Mix2 firmware installed, we'll use the Mix2 label, otherwise assume Sample
          if (!firmwareSupportsMix2()) {
            return sample
          } else {
            return mix2
          }
        }
      }
      return sample;
    },
  },
}
</script>

<style scoped>
.settingList > :nth-child(odd) {
  background-color: #353937;
}

.settingList > :nth-child(even) {
  background-color: #242826;
}

</style>
