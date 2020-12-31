import Vue, { VueConstructor } from 'vue';
import { Store } from 'vuex';
import { AppState } from '@/app/store';
import { ChatboxService } from '@/domain/chatbox/service';
import { BloomService } from '@/domain/bloom';

abstract class VueAppClass extends Vue {
  public $store!: Store<AppState>;
  public $chatbox!: ChatboxService;
  public $bloom!: BloomService;
}
export const VueApp = Vue as VueConstructor<VueAppClass>;
