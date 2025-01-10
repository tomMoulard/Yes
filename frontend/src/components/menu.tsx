import React from 'react';
import { Menu } from 'antd';

export default function MenuComponent() {
return (<Menu
    // items={items} // FIXME: add navigation items.
    mode="inline"
    style={{ flex: 1, minWidth: 0 }}
    theme="dark"
        />)
}

