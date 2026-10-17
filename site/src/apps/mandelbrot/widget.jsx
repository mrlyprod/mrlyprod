import { Scene } from '../../lib/app/page.jsx';
import { make } from './scene.js';
export const Widget = (props) => <Scene make={make} {...props} />;
