import { Scene } from '../../lib/scene.jsx';
import { make } from './scene.js';
export const Widget = (props) => <Scene make={make} {...props} />;
