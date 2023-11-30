import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const foxTheme: CustomThemeConfig = {
	name: 'fox-theme',
	properties: {
		// =~= Theme Properties =~=
		'--theme-font-family-base': `system-ui`,
		'--theme-font-family-heading': `system-ui`,
		'--theme-font-color-base': '0 0 0',
		'--theme-font-color-dark': '255 255 255',
		'--theme-rounded-base': '9999px',
		'--theme-rounded-container': '8px',
		'--theme-border-base': '1px',
		// =~= Theme On-X Colors =~=
		'--on-primary': '255 255 255',
		'--on-secondary': '0 0 0',
		'--on-tertiary': '0 0 0',
		'--on-success': '255 255 255',
		'--on-warning': '255 255 255',
		'--on-error': '0 0 0',
		'--on-surface': '0 0 0',
		// =~= Theme Colors  =~=
		// primary | #a51758
		'--color-primary-50': '242 220 230', // #f2dce6
		'--color-primary-100': '237 209 222', // #edd1de
		'--color-primary-200': '233 197 213', // #e9c5d5
		'--color-primary-300': '219 162 188', // #dba2bc
		'--color-primary-400': '192 93 138', // #c05d8a
		'--color-primary-500': '165 23 88', // #a51758
		'--color-primary-600': '149 21 79', // #95154f
		'--color-primary-700': '124 17 66', // #7c1142
		'--color-primary-800': '99 14 53', // #630e35
		'--color-primary-900': '81 11 43', // #510b2b
		// secondary | #827beb
		'--color-secondary-50': '236 235 252', // #ecebfc
		'--color-secondary-100': '230 229 251', // #e6e5fb
		'--color-secondary-200': '224 222 250', // #e0defa
		'--color-secondary-300': '205 202 247', // #cdcaf7
		'--color-secondary-400': '168 163 241', // #a8a3f1
		'--color-secondary-500': '130 123 235', // #827beb
		'--color-secondary-600': '117 111 212', // #756fd4
		'--color-secondary-700': '98 92 176', // #625cb0
		'--color-secondary-800': '78 74 141', // #4e4a8d
		'--color-secondary-900': '64 60 115', // #403c73
		// tertiary | #63e772
		'--color-tertiary-50': '232 251 234', // #e8fbea
		'--color-tertiary-100': '224 250 227', // #e0fae3
		'--color-tertiary-200': '216 249 220', // #d8f9dc
		'--color-tertiary-300': '193 245 199', // #c1f5c7
		'--color-tertiary-400': '146 238 156', // #92ee9c
		'--color-tertiary-500': '99 231 114', // #63e772
		'--color-tertiary-600': '89 208 103', // #59d067
		'--color-tertiary-700': '74 173 86', // #4aad56
		'--color-tertiary-800': '59 139 68', // #3b8b44
		'--color-tertiary-900': '49 113 56', // #317138
		// success | #3a4ac7
		'--color-success-50': '225 228 247', // #e1e4f7
		'--color-success-100': '216 219 244', // #d8dbf4
		'--color-success-200': '206 210 241', // #ced2f1
		'--color-success-300': '176 183 233', // #b0b7e9
		'--color-success-400': '117 128 216', // #7580d8
		'--color-success-500': '58 74 199', // #3a4ac7
		'--color-success-600': '52 67 179', // #3443b3
		'--color-success-700': '44 56 149', // #2c3895
		'--color-success-800': '35 44 119', // #232c77
		'--color-success-900': '28 36 98', // #1c2462
		// warning | #5d5efd
		'--color-warning-50': '231 231 255', // #e7e7ff
		'--color-warning-100': '223 223 255', // #dfdfff
		'--color-warning-200': '215 215 255', // #d7d7ff
		'--color-warning-300': '190 191 254', // #bebffe
		'--color-warning-400': '142 142 254', // #8e8efe
		'--color-warning-500': '93 94 253', // #5d5efd
		'--color-warning-600': '84 85 228', // #5455e4
		'--color-warning-700': '70 71 190', // #4647be
		'--color-warning-800': '56 56 152', // #383898
		'--color-warning-900': '46 46 124', // #2e2e7c
		// error | #e4ac56
		'--color-error-50': '251 243 230', // #fbf3e6
		'--color-error-100': '250 238 221', // #faeedd
		'--color-error-200': '248 234 213', // #f8ead5
		'--color-error-300': '244 222 187', // #f4debb
		'--color-error-400': '236 197 137', // #ecc589
		'--color-error-500': '228 172 86', // #e4ac56
		'--color-error-600': '205 155 77', // #cd9b4d
		'--color-error-700': '171 129 65', // #ab8141
		'--color-error-800': '137 103 52', // #896734
		'--color-error-900': '112 84 42', // #70542a
		// surface | #bee277
		'--color-surface-50': '245 251 235', // #f5fbeb
		'--color-surface-100': '242 249 228', // #f2f9e4
		'--color-surface-200': '239 248 221', // #eff8dd
		'--color-surface-300': '229 243 201', // #e5f3c9
		'--color-surface-400': '210 235 160', // #d2eba0
		'--color-surface-500': '190 226 119', // #bee277
		'--color-surface-600': '171 203 107', // #abcb6b
		'--color-surface-700': '143 170 89', // #8faa59
		'--color-surface-800': '114 136 71', // #728847
		'--color-surface-900': '93 111 58' // #5d6f3a
	}
};
