<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TS.AdmUpdateBuildingByID.024.002</name>
   <tag></tag>
   <elementGuidId>601d0573-c2ed-46d8-8a23-e1cd9cc9d6d5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.adminToken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;:\&quot;Building Update\&quot;,\n  \&quot;facilities\&quot;:[\n      {\n        \&quot;facilityId\&quot;:3,\n        \&quot;name\&quot;:\&quot;Facility 3\&quot;,\n        \&quot;icon\&quot;:\&quot;https://ik.imagekit.io/fortyfour/FacilityCategory/other_houses_FILL0_wght400_GRAD0_opsz48_ySm9eMGlM.svg\&quot;,\n        \&quot;iconName\&quot;:\&quot;other_houses\&quot;,\n        \&quot;description\&quot;:\&quot;Facility 3\&quot;\n      }],\n    \&quot;pictures\&quot;:[\n      {\n        \&quot;pictureId\&quot;:\&quot;789\&quot;,\n        \&quot;index\&quot;:0,\n        \&quot;url\&quot;:\&quot;https://ik.imagekit.io/fortyfour/default-image.jpg?ik-sdk-version\u003djavascript-1.4.3\\u0026updatedAt\u003d1651795109157\&quot;,\n        \&quot;alt\&quot;:\&quot;Picture 3\&quot;\n      }],\n    \&quot;description\&quot;:\&quot;Building 2\&quot;,\n    \&quot;capacity\&quot;:100,\n    \&quot;size\&quot;:100,\n    \&quot;price\&quot;:{\n      \&quot;annual\&quot;:100000,\n      \&quot;monthly\&quot;:10000\n    },\n   \&quot;location\&quot;:{\n      \&quot;address\&quot;:\&quot;Address 2\&quot;,\n      \&quot;city\&quot;:{\n        \&quot;id\&quot;:154,\n        \&quot;name\&quot;:\&quot;Kota Jakarta Selatan\&quot;\n      },\n      \&quot;district\&quot;:{\n        \&quot;id\&quot;:1885,\n        \&quot;cityId\&quot;:154,\n        \&quot;name\&quot;:\&quot;Pesanggrahan\&quot;\n      },\n      \&quot;geo\&quot;:{\n        \&quot;long\&quot;:0,\n        \&quot;lat\&quot;:0\n      }\n    },\n    \&quot;isPublished\&quot;:true\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>919214af-8437-4065-944c-6e9ece9f2400</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.adminToken}</value>
      <webElementGuid>37ec2bd0-0132-4048-8cb1-003f17a52424</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/admin/buildings/${buildingID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'${GlobalVariable.AbuildingID}'</defaultValue>
      <description></description>
      <id>75093f1c-3c9d-4577-997c-22ecdd060811</id>
      <masked>false</masked>
      <name>buildingID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
