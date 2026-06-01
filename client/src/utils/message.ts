/**
 * Clickable message wrapper — every toast is dismissible on click.
 *
 * Drop-in replacement for ant-design-vue's `message`:
 *   import { message } from '@/utils/message';
 *   message.success('done');
 *   message.error('failed');
 *   message.info({ content: 'hello', duration: 10 });
 */
import { message as rawMessage } from 'ant-design-vue';

function wrap(fn: (opts: any) => any) {
  return (opts: string | { content: string; [k: string]: any }) => {
    if (typeof opts === 'string') {
      const destroy = fn({ content: opts, onClick: () => destroy() });
      return destroy;
    }
    const destroy = fn({ ...opts, onClick: () => destroy() });
    return destroy;
  };
}

export const message = {
  success: wrap(rawMessage.success.bind(rawMessage)),
  error: wrap(rawMessage.error.bind(rawMessage)),
  info: wrap(rawMessage.info.bind(rawMessage)),
  warning: wrap(rawMessage.warning.bind(rawMessage)),
  warn: wrap((rawMessage.warn ?? rawMessage.warning).bind(rawMessage)),
  loading: wrap(rawMessage.loading.bind(rawMessage)),
  open: wrap(rawMessage.open.bind(rawMessage)),
  destroy: rawMessage.destroy.bind(rawMessage),
  config: rawMessage.config.bind(rawMessage),
};
